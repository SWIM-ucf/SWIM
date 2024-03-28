//! Implementation of a RISC-V floating-point coprocessor.

use super::constants::*;
use super::control_signals::floating_point::*;
use super::registers::FpRegisters;
use super::instruction::Instruction;
use serde::{Deserialize, Serialize};

/// An implementation of a floating-point coprocessor for the RISC-V ISA.
///
/// Different from the main processor, much of the functionality of the coprocessor
/// is controlled remotely using its available API calls.
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct RiscFpCoprocessor {
    instruction: Instruction,
    pub signals: FpuControlSignals,
    pub state: RiscFpuState,
    pub is_halted: bool,
    pub registers: FpRegisters,
    pub condition_code: u64,
    pub data: u64,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RiscFpuState {
    pub instruction: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub rs3: u32,
    pub rd: u32,
    pub shamt: u32,
    pub funct2: u32,
    pub funct3: u32,
    pub funct7: u32,
    pub imm: u32,
    pub imm1: u32,
    pub imm2: u32,
    pub branch_flag: bool,

    /// The line that comes out of the condition code register file. Should contain
    /// 1 for true or 0 for false.
    pub condition_code_bit: u8,
    /// The inversion of `condition_code_bit`.
    pub condition_code_bit_inverted: u8,
    /// The result of the multiplexer with `condition_code_bit` and `condition_code_bit_inverted`.
    pub condition_code_mux: u8,

    pub data_from_main_processor: u64,
    pub data_writeback: u64,
    pub destination: usize,
    pub fp_register_data_from_main_processor: u64,
    pub read_data_1: u64,
    pub read_data_2: u64,
    pub register_write_data: u64,
    pub register_write_mux_to_mux: u64,
    pub sign_extend_data: u64,

    /// Data line that goes from `Read Data 2` to the multiplexer in the main processor
    /// controlled by [`MemWriteSrc`](super::control_signals::MemWriteSrc).
    /// This variable in a way in just a copy of read_data_2
    pub fp_register_to_memory: u64,

    pub alu_result: u64,
    pub comparator_result: u64,
}

impl RiscFpCoprocessor {
    // ========================== Stages ==========================
    pub fn stage_instruction_decode(&mut self) {
        self.instruction_decode();
        self.set_control_signals();
        self.read_registers();
    }

    pub fn stage_execute(&mut self) {
        self.alu();
        self.comparator();
        self.write_condition_code();
        self.write_fp_register_to_memory();
        self.set_condition_code_line();
    }

    pub fn stage_memory(&mut self) {
        self.write_data();
        self.set_data_writeback();
        self.set_fpu_branch();
    }

    pub fn stage_writeback(&mut self) {
        self.register_write();
    }

    // ===================== General Functions =====================
    /// Handle an otherwise irrecoverable error within the datapath.
    pub fn error(&mut self, _message: &str) {
        self.is_halted = true;
    }

    // =================== API For Main Processor ===================
    /// Set the internally-stored copy of the current instruction. This effectively
    /// operates in lieu of any "instruction fetch" functionality since the coprocessor
    /// does not fetch instructions.
    pub fn set_instruction(&mut self, instruction_bits: u32) {
        self.state.instruction = instruction_bits;
        if let Ok(instruction) = Instruction::try_from(self.state.instruction) {
            self.instruction = instruction;
        }
    }

    /// Sets the data line between the main processor and the `Data` register. This
    /// is then used if deciding data from the main processor should go into the `Data`
    /// register.
    pub fn set_data_from_main_processor(&mut self, data: u64) {
        self.state.data_from_main_processor = data;
    }

    /// Gets the contents of the data line between the `Data` register and the multiplexer
    /// in the main processor controlled by the [`DataWrite`] control signal.
    pub fn get_data_writeback(&mut self) -> u64 {
        self.state.data_writeback
    }

    /// Sets the data line between the multiplexer controlled by [`MemToReg`](super::control_signals::MemToReg)
    /// in the main processor and the multiplexer controlled by [`FpuMemToReg`] in the
    /// floating-point coprocessor.
    pub fn set_fp_register_data_from_main_processor(&mut self, data: u64) {
        self.state.fp_register_data_from_main_processor = data;
    }

    /// Gets the contents of the data line that goes from `Read Data 2` to the multiplexer
    /// in the main processor controlled by [`MemWriteSrc`](super::control_signals::MemWriteSrc).
    pub fn get_fp_register_to_memory(&mut self) -> u64 {
        self.state.fp_register_to_memory
    }

    pub fn set_register(&mut self, _register: usize, _data: u64) -> Result<(), String> {
        if _register >= 32 {
            return Err(format!("Register index out of bounds: {}", _register));
        }

        let register = &mut self.registers.fpr[_register];
        *register = _data;

        Ok(())
    }

    pub fn register_from_str(&self, _register: &str) -> Option<usize> {
        // Check if register matches a register between f0 and f31
        if _register.len() >= 2 && &_register[0..1] == "f" && _register.len() <= 3 {
            let register = &_register[1..];
            if let Ok(register) = register.parse::<usize>() {
                if register < 32 {
                    return Some(register);
                }
            }
        }
        None
    }

    // ================== Instruction Decode (ID) ==================
    /// Decode an instruction into its individual fields.
    fn instruction_decode(&mut self) {
        // Set the data lines based on the contents of the instruction.
        // Some lines will hold uninitialized values as a result.
        match self.instruction {
            Instruction::RType(r) => {
                self.state.rs1 = r.rs1 as u32;
                self.state.rs2 = r.rs2 as u32;
                self.state.rd = r.rd as u32;
                self.state.shamt = r.rs2 as u32;
                self.state.funct3 = r.funct3 as u32;
                self.state.funct7 = r.funct7 as u32;
            }
            Instruction::IType(i) => {
                self.state.rs1 = i.rs1 as u32;
                self.state.funct3 = i.funct3 as u32;
                self.state.rd = i.rd as u32;
                self.state.imm = i.imm as u32;
                self.state.shamt = (i.imm & 0x003f) as u32;
            }
            Instruction::SType(s) => {
                self.state.rs2 = s.rs2 as u32;
                self.state.rs1 = s.rs1 as u32;
                self.state.funct3 = s.funct3 as u32;
                self.state.imm1 = s.imm1 as u32;
                self.state.imm2 = s.imm2 as u32;
            }
            Instruction::R4Type(r4) => {
                self.state.rs3 = r4.rs3 as u32;
                self.state.funct2 = r4.funct2 as u32;
                self.state.rs2 = r4.rs2 as u32;
                self.state.rs1 = r4.rs1 as u32;
                self.state.funct3 = r4.funct3 as u32;
                self.state.rd = r4.rd as u32;
            }
            _ => (),
        }
    }

    /// Set the control signals of the processor based on the instruction opcode and function
    /// control signals.
    fn set_control_signals(&mut self) {
        match self.instruction {
            Instruction::RType(r) => {
                self.set_rtype_control_signals(r);
            }
            Instruction::IType(i) => {
                self.signals = FpuControlSignals {
                    data_write: DataWrite::NoWrite,
                    fpu_branch: FpuBranch::NoBranch,
                    fpu_mem_to_reg: FpuMemToReg::UseMemory,
                    fpu_reg_dst: FpuRegDst::Reg3,
                    fpu_reg_write: FpuRegWrite::YesWrite,
                    ..Default::default()
                }
            }
            Instruction::SType(s) => {
                self.set_stype_control_signals(s);
            }
            Instruction::BType(b) => {
                self.set_btype_control_signals(b);
            }
            Instruction::UType(u) => {
                self.set_utype_control_signals(u);
            }
            Instruction::JType(j) => {
                self.set_jtype_control_signals(j);
            }
            _ => self.error("Unsupported Instruction!"),
        }
    }

    /// Read the registers as specified from the instruction and pass
    /// the data into the datapath.
    fn read_registers(&mut self) {
        let reg1 = self.state.rs1 as usize;
        let reg2 = self.state.rs2 as usize;

        self.state.read_data_1 = self.registers.fpr[reg1];
        self.state.read_data_2 = self.registers.fpr[reg2];
    }

    // ======================= Execute (EX) =======================
    /// Perform an ALU operation.
    fn alu(&mut self) {
        let input1 = self.state.read_data_1;
        let input2 = self.state.read_data_2;

        let mut input1_f32 = 0f32;
        let mut input2_f32 = 0f32;
        let mut input1_f64 = 0f64;
        let mut input2_f64 = 0f64;

        // Truncate the inputs if 32-bit operations are expected.
        if let FpuRegWidth::Word = self.signals.fpu_reg_width {
            input1_f32 = f32::from_bits(input1 as u32);
            input2_f32 = f32::from_bits(input2 as u32);
        } else {
            input1_f64 = f64::from_bits(input1);
            input2_f64 = f64::from_bits(input2);
        }

        self.state.alu_result = match self.signals.fpu_alu_op {
            FpuAluOp::Addition => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => f32::to_bits(input1_f32 + input2_f32) as u64,
                FpuRegWidth::DoubleWord => f64::to_bits(input1_f64 + input2_f64),
            },
            FpuAluOp::Subtraction => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => f32::to_bits(input1_f32 - input2_f32) as u64,
                FpuRegWidth::DoubleWord => f64::to_bits(input1_f64 - input2_f64),
            },
            FpuAluOp::MultiplicationOrEqual => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => f32::to_bits(input1_f32 * input2_f32) as u64,
                FpuRegWidth::DoubleWord => f64::to_bits(input1_f64 * input2_f64),
            },
            FpuAluOp::Division => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => {
                    if input2_f32 == 0f32 {
                        f32::to_bits(0f32) as u64
                    } else {
                        f32::to_bits(input1_f32 / input2_f32) as u64
                    }
                }
                FpuRegWidth::DoubleWord => {
                    if input2_f64 == 0.0 {
                        f64::to_bits(0.0)
                    } else {
                        f64::to_bits(input1_f64 / input2_f64)
                    }
                }
            },
            // No operation.
            FpuAluOp::Slt | FpuAluOp::Snge | FpuAluOp::Sle | FpuAluOp::Sngt => 0,
            _ => {
                self.error(&format!(
                    "Unsupported operation in FPU `{:?}`",
                    self.signals.fpu_alu_op
                ));
                0
            }
        };
    }

    /// Perform a comparison.
    fn comparator(&mut self) {
        let input1 = self.state.read_data_1;
        let input2 = self.state.read_data_2;

        let input1_f32 = f32::from_bits(input1 as u32);
        let input2_f32 = f32::from_bits(input2 as u32);
        let input1_f64 = f64::from_bits(input1);
        let input2_f64 = f64::from_bits(input2);

        self.state.comparator_result = match self.signals.fpu_alu_op {
            FpuAluOp::MultiplicationOrEqual => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => (input1_f32 == input2_f32) as u64,
                FpuRegWidth::DoubleWord => (input1_f64 == input2_f64) as u64,
            },
            FpuAluOp::Slt => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => (input1_f32 < input2_f32) as u64,
                FpuRegWidth::DoubleWord => (input1_f64 < input2_f64) as u64,
            },
            FpuAluOp::Sle => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => (input1_f32 <= input2_f32) as u64,
                FpuRegWidth::DoubleWord => (input1_f64 <= input2_f64) as u64,
            },
            FpuAluOp::Sngt => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => !input1_f32.gt(&input2_f32) as u64,
                FpuRegWidth::DoubleWord => !input1_f64.gt(&input2_f64) as u64,
            },
            FpuAluOp::Snge => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => !input1_f32.ge(&input2_f32) as u64,
                FpuRegWidth::DoubleWord => !input1_f64.ge(&input2_f64) as u64,
            },
            FpuAluOp::Addition | FpuAluOp::Subtraction | FpuAluOp::Division => 0, // No operation
            _ => {
                self.error(&format!(
                    "Unsupported operation in comparator `{:?}`",
                    self.signals.fpu_alu_op
                ));
                0
            }
        }
    }

    /// Write to the `Data` register. This register is used to transfer data between
    /// the main processor and the coprocessor.
    fn write_data(&mut self) {
        if let DataWrite::NoWrite = self.signals.data_write {
            return;
        }

        self.data = match self.signals.data_src {
            DataSrc::FloatingPointUnit => self.state.read_data_1,
            DataSrc::MainProcessorUnit => self.state.data_from_main_processor,
        };
    }

    /// Set the condition code (CC) register based on the result from the comparator.
    fn write_condition_code(&mut self) {
        if let CcWrite::YesWrite = self.signals.cc_write {
            self.condition_code = self.state.comparator_result;
        }
    }

    /// Set the data line that goes from `Read Data 2` to the multiplexer in the main processor
    /// controlled by [`MemWriteSrc`](super::control_signals::MemWriteSrc).
    fn write_fp_register_to_memory(&mut self) {
        self.state.fp_register_to_memory = self.state.read_data_2;
    }

    // ======================= Memory (MEM) =======================
    /// Set the data line that goes out of the condition code register file.
    fn set_condition_code_line(&mut self) {
        // The MIPS architecture supports more than one condition code, but SWIM
        // manually uses only one. This stubs the possible use of more than one
        // for future development.
        let selected_register_data = match self.signals.cc {
            Cc::Cc0 => self.condition_code,
        };

        // This only considers one bit of the selected condition code register.
        self.state.condition_code_bit = match selected_register_data % 2 {
            0 => 0,
            _ => 1,
        };
    }

    /// Set the data line between the multiplexer after the `Data` register and the
    /// multiplexer in the main processor controlled by the [`DataWrite`] control signal.
    fn set_data_writeback(&mut self) {
        self.state.sign_extend_data = self.data as i32 as i64 as u64;
        self.state.data_writeback = match self.signals.fpu_reg_width {
            FpuRegWidth::Word => self.state.sign_extend_data,
            FpuRegWidth::DoubleWord => self.data,
        }
    }

    /// Simulate the logic between `self.state.condition_code_bit` and the FPU branch
    /// AND gate.
    fn set_fpu_branch(&mut self) {
        // Invert the condition code. (In this case, instead of using a bitwise NOT, this
        // will invert only the last digit and leave the rest as 0.)
        self.state.condition_code_bit_inverted = match self.state.condition_code_bit % 2 {
            0 => 1,
            _ => 0,
        };

        // Run the multiplexer.
        self.state.condition_code_mux = match self.state.branch_flag {
            // 0 - Use inverted condition code.
            false => self.state.condition_code_bit_inverted,
            // 1 - Use condition code value as-is.
            true => self.state.condition_code_bit,
        };

        // Set the result of the AND gate.
        self.signals.fpu_take_branch = if self.signals.fpu_branch == FpuBranch::YesBranch
            && self.state.condition_code_mux == 1
        {
            FpuTakeBranch::YesBranch
        } else {
            FpuTakeBranch::NoBranch
        };
    }

    // ====================== Writeback (WB) ======================
    /// Write data to the floating-point register file.
    fn register_write(&mut self) {
        if let FpuRegWrite::NoWrite = self.signals.fpu_reg_write {
            return;
        }

        self.state.destination = match self.signals.fpu_reg_dst {
            FpuRegDst::Reg1 => self.state.rs1 as usize,
            FpuRegDst::Reg2 => self.state.rs2 as usize,
            FpuRegDst::Reg3 => self.state.rd as usize,
        };

        self.state.register_write_mux_to_mux = match self.signals.data_write {
            DataWrite::NoWrite => self.state.alu_result,
            DataWrite::YesWrite => self.data,
        };
        self.state.register_write_data = match self.signals.fpu_mem_to_reg {
            FpuMemToReg::UseDataWrite => self.state.register_write_mux_to_mux,
            FpuMemToReg::UseMemory => self.state.fp_register_data_from_main_processor,
        };
        self.registers.fpr[self.state.destination] = self.state.register_write_data;
    }
}
