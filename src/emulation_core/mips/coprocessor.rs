//! Implementation of a MIPS64 floating-point coprocessor.

use super::constants::*;
use super::control_signals::floating_point::*;
use super::instruction::Instruction;

/// An implementation of a floating-point coprocessor for the MIPS64 ISA.
///
/// Different from the main processor, much of the functionality of the coprocessor
/// is controlled remotely using its available API calls.
#[derive(Clone, Default, PartialEq)]
pub struct MipsFpCoprocessor {
    instruction: Instruction,
    pub signals: FpuControlSignals,
    pub state: FpuState,

    pub fpr: [u64; 32],
    pub condition_code: u64,
    data: u64,
}

#[derive(Clone, Default, PartialEq)]
pub struct FpuState {
    pub instruction: u32,
    pub op: u32,
    pub fmt: u32,
    pub fs: u32,
    pub ft: u32,
    pub fd: u32,
    pub function: u32,

    pub data_from_main_processor: u64,
    pub fp_register_data_from_main_processor: u64,
    pub read_data_1: u64,
    pub read_data_2: u64,
    /// Data line that goes from `Read Data 2` to the multiplexer in the main processor
    /// controlled by [`MemWriteSrc`](super::control_signals::MemWriteSrc).
    pub fp_register_to_memory: u64,
    pub alu_result: u64,
    pub comparator_result: u64,
}

impl MipsFpCoprocessor {
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
    }

    pub fn stage_memory(&mut self) {
        self.write_data();
    }

    pub fn stage_writeback(&mut self) {
        self.register_write();
    }

    /// Set the internally-stored copy of the current instruction. This effectively
    /// operates in lieu of any "instruction fetch" functionality since the coprocessor
    /// does not fetch instructions.
    pub fn set_instruction(&mut self, instruction_bits: u32) {
        self.state.instruction = instruction_bits;
        self.instruction = Instruction::from(self.state.instruction);
    }

    /// Decode an instruction into its individual fields.
    fn instruction_decode(&mut self) {
        // Set the data lines based on the contents of the instruction.
        // Some lines will hold uninitialized values as a result.
        match self.instruction {
            Instruction::FpuRType(r) => {
                self.state.op = r.op as u32;
                self.state.fmt = r.fmt as u32;
                self.state.fs = r.fs as u32;
                self.state.ft = r.ft as u32;
                self.state.fd = r.fd as u32;
                self.state.function = r.function as u32;
            }
            Instruction::FpuIType(i) => {
                self.state.ft = i.ft as u32;
            }
            Instruction::FpuRegImmType(i) => {
                self.state.op = i.op as u32;
                self.state.fmt = 0; // Not applicable
                self.state.fs = i.fs as u32;
                self.state.ft = 0; // Not applicable
                self.state.fd = 0; // Not applicable
            }
            Instruction::FpuCompareType(c) => {
                self.state.op = c.op as u32;
                self.state.fmt = c.fmt as u32;
                self.state.ft = c.ft as u32;
                self.state.fs = c.fs as u32;
                self.state.function = c.function as u32;
            }
            // These types do not use the floating-point unit so they can be ignored.
            Instruction::RType(_) | Instruction::IType(_) | Instruction::JType(_) => (),
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
    pub fn get_data_register(&mut self) -> u64 {
        match self.signals.fpu_reg_width {
            FpuRegWidth::Word => self.data as i32 as i64 as u64,
            FpuRegWidth::DoubleWord => self.data,
        }
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

    /// Set the control signals of the processor based on the instruction opcode and function
    /// control signals.
    fn set_control_signals(&mut self) {
        match self.instruction {
            Instruction::FpuRType(r) => {
                match r.op {
                    OPCODE_COP1 => match r.function {
                        FUNCTION_ADD => {
                            self.signals.cc = Cc::Cc0;
                            self.signals.cc_write = CcWrite::NoWrite;
                            self.signals.data_src = DataSrc::FloatingPointUnit;
                            self.signals.data_write = DataWrite::NoWrite;
                            self.signals.fpu_alu_op = FpuAluOp::AdditionOrEqual;
                            self.signals.fpu_branch = FpuBranch::NoBranch;
                            self.signals.fpu_mem_to_reg = FpuMemToReg::UseDataWrite;
                            self.signals.fpu_reg_dst = FpuRegDst::Reg3;
                            self.signals.fpu_reg_width = FpuRegWidth::from_fmt(r.fmt);
                            self.signals.fpu_reg_write = FpuRegWrite::YesWrite;
                        }
                        FUNCTION_SUB => {
                            self.signals.cc = Cc::Cc0;
                            self.signals.cc_write = CcWrite::NoWrite;
                            self.signals.data_src = DataSrc::FloatingPointUnit;
                            self.signals.data_write = DataWrite::NoWrite;
                            self.signals.fpu_alu_op = FpuAluOp::Subtraction;
                            self.signals.fpu_branch = FpuBranch::NoBranch;
                            self.signals.fpu_mem_to_reg = FpuMemToReg::UseDataWrite;
                            self.signals.fpu_reg_dst = FpuRegDst::Reg3;
                            self.signals.fpu_reg_width = FpuRegWidth::from_fmt(r.fmt);
                            self.signals.fpu_reg_write = FpuRegWrite::YesWrite;
                        }
                        FUNCTION_MUL => {
                            self.signals.cc = Cc::Cc0;
                            self.signals.cc_write = CcWrite::NoWrite;
                            self.signals.data_src = DataSrc::FloatingPointUnit;
                            self.signals.data_write = DataWrite::NoWrite;
                            self.signals.fpu_alu_op = FpuAluOp::MultiplicationOrSlt;
                            self.signals.fpu_branch = FpuBranch::NoBranch;
                            self.signals.fpu_mem_to_reg = FpuMemToReg::UseDataWrite;
                            self.signals.fpu_reg_dst = FpuRegDst::Reg3;
                            self.signals.fpu_reg_width = FpuRegWidth::from_fmt(r.fmt);
                            self.signals.fpu_reg_write = FpuRegWrite::YesWrite;
                        }
                        FUNCTION_DIV => {
                            self.signals.cc = Cc::Cc0;
                            self.signals.cc_write = CcWrite::NoWrite;
                            self.signals.data_src = DataSrc::FloatingPointUnit;
                            self.signals.data_write = DataWrite::NoWrite;
                            self.signals.fpu_alu_op = FpuAluOp::DivisionOrSle;
                            self.signals.fpu_branch = FpuBranch::NoBranch;
                            self.signals.fpu_mem_to_reg = FpuMemToReg::UseDataWrite;
                            self.signals.fpu_reg_dst = FpuRegDst::Reg3;
                            self.signals.fpu_reg_width = FpuRegWidth::from_fmt(r.fmt);
                            self.signals.fpu_reg_write = FpuRegWrite::YesWrite;
                        }
                        // Unrecognized format code. Perform no operation.
                        _ => unimplemented!("COP1 instruction with function code `{}`", r.function),
                    },
                    // Unrecognized opcode. Perform no operation.
                    _ => unimplemented!("Unsupported opcode `{}` for FPU R-type instruction", r.op),
                }
            }
            Instruction::FpuIType(i) => match i.op {
                OPCODE_SWC1 => {
                    self.signals = FpuControlSignals {
                        cc_write: CcWrite::NoWrite,
                        data_write: DataWrite::NoWrite,
                        fpu_branch: FpuBranch::NoBranch,
                        fpu_reg_width: FpuRegWidth::Word,
                        fpu_reg_write: FpuRegWrite::NoWrite,
                        ..Default::default()
                    }
                }
                OPCODE_LWC1 => {
                    self.signals = FpuControlSignals {
                        cc_write: CcWrite::NoWrite,
                        data_write: DataWrite::NoWrite,
                        fpu_branch: FpuBranch::NoBranch,
                        fpu_mem_to_reg: FpuMemToReg::UseMemory,
                        fpu_reg_dst: FpuRegDst::Reg1,
                        fpu_reg_width: FpuRegWidth::Word,
                        fpu_reg_write: FpuRegWrite::YesWrite,
                        ..Default::default()
                    }
                }
                _ => unimplemented!("Unsupported opcode `{}` for FPU I-type instruction", i.op),
            },
            Instruction::FpuRegImmType(i) => match i.sub {
                SUB_MT => {
                    self.signals = FpuControlSignals {
                        cc_write: CcWrite::NoWrite,
                        data_src: DataSrc::MainProcessorUnit,
                        data_write: DataWrite::YesWrite,
                        fpu_branch: FpuBranch::NoBranch,
                        fpu_mem_to_reg: FpuMemToReg::UseDataWrite,
                        fpu_reg_dst: FpuRegDst::Reg2,
                        fpu_reg_width: FpuRegWidth::Word,
                        fpu_reg_write: FpuRegWrite::YesWrite,
                        ..Default::default()
                    }
                }
                SUB_DMT => {
                    self.signals = FpuControlSignals {
                        cc_write: CcWrite::NoWrite,
                        data_src: DataSrc::MainProcessorUnit,
                        data_write: DataWrite::YesWrite,
                        fpu_branch: FpuBranch::NoBranch,
                        fpu_mem_to_reg: FpuMemToReg::UseDataWrite,
                        fpu_reg_dst: FpuRegDst::Reg2,
                        fpu_reg_width: FpuRegWidth::DoubleWord,
                        fpu_reg_write: FpuRegWrite::YesWrite,
                        ..Default::default()
                    }
                }
                SUB_MF => {
                    self.signals = FpuControlSignals {
                        cc_write: CcWrite::NoWrite,
                        data_src: DataSrc::FloatingPointUnit,
                        data_write: DataWrite::YesWrite,
                        fpu_branch: FpuBranch::NoBranch,
                        fpu_reg_width: FpuRegWidth::Word,
                        fpu_reg_write: FpuRegWrite::NoWrite,
                        ..Default::default()
                    }
                }
                SUB_DMF => {
                    self.signals = FpuControlSignals {
                        cc_write: CcWrite::NoWrite,
                        data_src: DataSrc::FloatingPointUnit,
                        data_write: DataWrite::YesWrite,
                        fpu_branch: FpuBranch::NoBranch,
                        fpu_reg_width: FpuRegWidth::DoubleWord,
                        fpu_reg_write: FpuRegWrite::NoWrite,
                        ..Default::default()
                    }
                }
                _ => unimplemented!(
                    "Unsupported sub code `{}` for FPU register-immediate instruction",
                    i.sub
                ),
            },
            Instruction::FpuCompareType(c) => {
                self.signals = FpuControlSignals {
                    cc: Cc::Cc0,
                    cc_write: CcWrite::YesWrite,
                    data_write: DataWrite::NoWrite,
                    fpu_alu_op: FpuAluOp::from_function(c.function),
                    fpu_branch: FpuBranch::NoBranch,
                    fpu_reg_width: FpuRegWidth::from_fmt(c.fmt),
                    fpu_reg_write: FpuRegWrite::NoWrite,
                    ..Default::default()
                }
            }
            // These types do not use the floating-point unit so they can be ignored.
            Instruction::RType(_) | Instruction::IType(_) | Instruction::JType(_) => {
                self.signals = FpuControlSignals::default()
            }
        }
    }

    /// Read the registers as specified from the instruction and pass
    /// the data into the datapath.
    fn read_registers(&mut self) {
        let reg1 = self.state.fs as usize;
        let reg2 = self.state.ft as usize;

        self.state.read_data_1 = self.fpr[reg1];
        self.state.read_data_2 = self.fpr[reg2];

        // Truncate the variable data if a 32-bit word is requested.
        if let FpuRegWidth::Word = self.signals.fpu_reg_width {
            self.state.read_data_1 = self.fpr[reg1] as u32 as u64;
            self.state.read_data_2 = self.fpr[reg2] as u32 as u64;
        }
    }

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
            FpuAluOp::AdditionOrEqual => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => f32::to_bits(input1_f32 + input2_f32) as u64,
                FpuRegWidth::DoubleWord => f64::to_bits(input1_f64 + input2_f64),
            },
            FpuAluOp::Subtraction => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => f32::to_bits(input1_f32 - input2_f32) as u64,
                FpuRegWidth::DoubleWord => f64::to_bits(input1_f64 - input2_f64),
            },
            FpuAluOp::MultiplicationOrSlt => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => f32::to_bits(input1_f32 * input2_f32) as u64,
                FpuRegWidth::DoubleWord => f64::to_bits(input1_f64 * input2_f64),
            },
            FpuAluOp::DivisionOrSle => match self.signals.fpu_reg_width {
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
            FpuAluOp::Sngt | FpuAluOp::Snge => 0,
            _ => unimplemented!(),
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
            FpuAluOp::AdditionOrEqual => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => (input1_f32 == input2_f32) as u64,
                FpuRegWidth::DoubleWord => (input1_f64 == input2_f64) as u64,
            },
            FpuAluOp::MultiplicationOrSlt => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => (input1_f32 < input2_f32) as u64,
                FpuRegWidth::DoubleWord => (input1_f64 < input2_f64) as u64,
            },
            FpuAluOp::DivisionOrSle => match self.signals.fpu_reg_width {
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
            FpuAluOp::Subtraction => 0, // No operation
            _ => unimplemented!(),
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
        self.condition_code = self.state.comparator_result;
    }

    /// Set the data line that goes from `Read Data 2` to the multiplexer in the main processor
    /// controlled by [`MemWriteSrc`](super::control_signals::MemWriteSrc).
    fn write_fp_register_to_memory(&mut self) {
        self.state.fp_register_to_memory = self.state.read_data_2;
    }

    /// Write data to the floating-point register file.
    fn register_write(&mut self) {
        if let FpuRegWrite::NoWrite = self.signals.fpu_reg_write {
            return;
        }

        let destination = match self.signals.fpu_reg_dst {
            FpuRegDst::Reg1 => self.state.ft as usize,
            FpuRegDst::Reg2 => self.state.fs as usize,
            FpuRegDst::Reg3 => self.state.fd as usize,
        };

        let register_data = match self.signals.fpu_mem_to_reg {
            FpuMemToReg::UseDataWrite => match self.signals.data_write {
                DataWrite::NoWrite => self.state.alu_result,
                DataWrite::YesWrite => self.data,
            },
            FpuMemToReg::UseMemory => self.state.fp_register_data_from_main_processor,
        };

        self.fpr[destination] = register_data;
    }
}
