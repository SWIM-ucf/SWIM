//! Implementation of a RISC-V floating-point coprocessor.

use std::ops::Neg;

use super::constants::*;
use super::instruction::RiscInstruction;
use super::registers::RiscFpRegisters;
use super::{constants::RISC_NAN, control_signals::floating_point::*};
use serde::{Deserialize, Serialize};

/// An implementation of a floating-point coprocessor for the RISC-V ISA.
///
/// Different from the main processor, much of the functionality of the coprocessor
/// is controlled remotely using its available API calls.
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct RiscFpCoprocessor {
    instruction: RiscInstruction,
    pub signals: FpuControlSignals,
    pub state: RiscFpuState,
    pub is_halted: bool,
    pub registers: RiscFpRegisters,
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

    pub data_from_main_processor: u64,
    pub data_writeback: u64,
    pub destination: usize,
    pub fp_register_data_from_main_processor: u64,
    pub read_data_1: u64,
    pub read_data_2: u64,
    pub read_data_3: u64,
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
        self.write_fp_register_to_memory();
    }

    pub fn stage_memory(&mut self) {
        self.write_data();
        self.set_data_writeback();
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
        if let Ok(instruction) = RiscInstruction::try_from(self.state.instruction) {
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

    /// Sets the data line between the multiplexer controlled by [`WBSel`](super::control_signals::WBSel)
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
            RiscInstruction::RType(r) => {
                self.state.rs1 = r.rs1 as u32;
                self.state.rs2 = r.rs2 as u32;
                self.state.rd = r.rd as u32;
                self.state.shamt = r.rs2 as u32;
                self.state.funct3 = r.funct3 as u32;
                self.state.funct7 = r.funct7 as u32;
            }
            RiscInstruction::IType(i) => {
                self.state.rs1 = i.rs1 as u32;
                self.state.funct3 = i.funct3 as u32;
                self.state.rd = i.rd as u32;
                self.state.imm = i.imm as u32;
                self.state.shamt = (i.imm & 0x003f) as u32;
            }
            RiscInstruction::SType(s) => {
                self.state.rs2 = s.rs2 as u32;
                self.state.rs1 = s.rs1 as u32;
                self.state.funct3 = s.funct3 as u32;
                self.state.imm1 = s.imm1 as u32;
                self.state.imm2 = s.imm2 as u32;
            }
            RiscInstruction::R4Type(r4) => {
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
            RiscInstruction::RType(r) => {
                self.signals = FpuControlSignals {
                    data_write: DataWrite::NoWrite,
                    fpu_mem_to_reg: FpuMemToReg::UseDataWrite,
                    fpu_reg_write: FpuRegWrite::YesWrite,
                    fpu_reg_dst: FpuRegDst::Reg3,
                    ..Default::default()
                };

                if r.op != OPCODE_OP_FP {
                    self.signals.fpu_reg_write = FpuRegWrite::NoWrite;
                    return;
                }

                match r.funct7 >> 2 {
                    0 => self.signals.fpu_alu_op = FpuAluOp::Addition,
                    1 => self.signals.fpu_alu_op = FpuAluOp::Subtraction,
                    2 => self.signals.fpu_alu_op = FpuAluOp::MultiplicationOrEqual,
                    3 => self.signals.fpu_alu_op = FpuAluOp::Division,
                    4 => match r.funct3 {
                        0 => self.signals.fpu_alu_op = FpuAluOp::SGNJ,
                        1 => self.signals.fpu_alu_op = FpuAluOp::SGNJN,
                        2 => self.signals.fpu_alu_op = FpuAluOp::SGNJX,
                        _ => self.error("Unsupported Instruction!"),
                    },
                    5 => match r.funct3 {
                        0 => self.signals.fpu_alu_op = FpuAluOp::Min,
                        1 => self.signals.fpu_alu_op = FpuAluOp::Max,
                        _ => self.error("Unsupported Instruction!"),
                    },
                    11 => self.signals.fpu_alu_op = FpuAluOp::Sqrt,
                    20 => {
                        self.signals.fpu_reg_write = FpuRegWrite::NoWrite;
                        self.signals.data_write = DataWrite::YesWrite;
                        self.signals.data_src = DataSrc::FloatingPointUnitComp;
                        match r.funct3 {
                            0 => self.signals.fpu_alu_op = FpuAluOp::Sle,
                            1 => self.signals.fpu_alu_op = FpuAluOp::Slt,
                            2 => self.signals.fpu_alu_op = FpuAluOp::MultiplicationOrEqual,
                            _ => self.error("Unsupported Instruction!"),
                        }
                    }
                    24 => {
                        self.signals.data_write = DataWrite::YesWrite;
                        self.signals.fpu_reg_write = FpuRegWrite::NoWrite;
                    }
                    26 => {
                        self.signals.data_write = DataWrite::YesWrite;
                        self.signals.data_src = DataSrc::MainProcessorUnit;
                    }
                    28 => {
                        self.signals.data_write = DataWrite::YesWrite;
                        self.signals.fpu_reg_write = FpuRegWrite::NoWrite;
                        match r.funct3 {
                            0 => self.signals.data_src = DataSrc::FloatingPointBits,
                            1 => {
                                self.signals.fpu_alu_op = FpuAluOp::Class;
                                self.signals.data_src = DataSrc::FloatingPointUnitMask;
                            }
                            _ => self.error("Unsupported Instruction!"),
                        }
                    }
                    30 => {
                        self.signals.data_write = DataWrite::YesWrite;
                        self.signals.fpu_reg_write = FpuRegWrite::YesWrite;
                        self.signals.data_src = DataSrc::MainProcessorBits;
                    }
                    _ => self.error("Unsupported Instruction!"),
                }

                match r.funct3 {
                    0 => self.signals.round_mode = RoundingMode::RNE,
                    1 => self.signals.round_mode = RoundingMode::RTZ,
                    2 => self.signals.round_mode = RoundingMode::RDN,
                    3 => self.signals.round_mode = RoundingMode::RUP,
                    4 => self.signals.round_mode = RoundingMode::RMM,
                    7 => self.signals.round_mode = RoundingMode::DRM,
                    _ => self.error("Unsupported Rounding Mode!"),
                }
            }
            RiscInstruction::IType(i) => {
                self.signals = FpuControlSignals {
                    data_write: DataWrite::NoWrite,
                    fpu_mem_to_reg: FpuMemToReg::UseMemory,
                    fpu_reg_dst: FpuRegDst::Reg3,
                    fpu_reg_write: FpuRegWrite::YesWrite,
                    ..Default::default()
                };

                if i.op != OPCODE_LOAD_FP {
                    self.signals.fpu_reg_write = FpuRegWrite::NoWrite;
                }
            }
            RiscInstruction::SType(_s) => {
                self.signals = FpuControlSignals {
                    data_write: DataWrite::NoWrite,
                    fpu_reg_write: FpuRegWrite::NoWrite,
                    ..Default::default()
                };
            }
            RiscInstruction::R4Type(r4) => {
                self.signals = FpuControlSignals {
                    data_write: DataWrite::NoWrite,
                    fpu_mem_to_reg: FpuMemToReg::UseDataWrite,
                    fpu_reg_dst: FpuRegDst::Reg3,
                    fpu_reg_write: FpuRegWrite::YesWrite,
                    ..Default::default()
                };

                self.signals.fpu_alu_op = match r4.op {
                    OPCODE_MADD => FpuAluOp::MAdd,
                    OPCODE_MSUB => FpuAluOp::MSub,
                    OPCODE_NMSUB => FpuAluOp::NMSub,
                    OPCODE_NMADD => FpuAluOp::NMAdd,
                    _ => {
                        self.error("Unsupported Instruction!");
                        FpuAluOp::Addition
                    }
                };
            }
            _ => (),
        }
    }

    /// Read the registers as specified from the instruction and pass
    /// the data into the datapath.
    fn read_registers(&mut self) {
        let reg1 = self.state.rs1 as usize;
        let reg2 = self.state.rs2 as usize;
        let reg3 = self.state.rs3 as usize;

        self.state.read_data_1 = self.registers.fpr[reg1];
        self.state.read_data_2 = self.registers.fpr[reg2];
        self.state.read_data_3 = self.registers.fpr[reg3];
    }

    // ======================= Execute (EX) =======================
    /// Perform an ALU operation.
    fn alu(&mut self) {
        let input1 = self.state.read_data_1 as u32;
        let input2 = self.state.read_data_2 as u32;
        let input3 = self.state.read_data_3 as u32;
        let input1_f32 = f32::from_bits(input1);
        let input2_f32 = f32::from_bits(input2);
        let input3_f32 = f32::from_bits(input3);
        let mut input_mask = 0b0000000000;
        let input1_wo_sign = input1 & 0x7fffffff;
        let mut sign_bit = 0;

        let result_f32: f32 = match self.signals.fpu_alu_op {
            FpuAluOp::Addition => input1_f32 + input2_f32,
            FpuAluOp::Subtraction => input1_f32 - input2_f32,
            FpuAluOp::MultiplicationOrEqual => input1_f32 * input2_f32,
            FpuAluOp::Division => {
                if input2_f32 == 0.0 {
                    0.0
                } else {
                    input1_f32 / input2_f32
                }
            }
            FpuAluOp::Sqrt => input1_f32.sqrt(),
            FpuAluOp::Min => {
                if input1_f32 < input2_f32 {
                    input1_f32
                } else {
                    input2_f32
                }
            }
            FpuAluOp::Max => {
                if input1_f32 > input2_f32 {
                    input1_f32
                } else {
                    input2_f32
                }
            }
            FpuAluOp::SGNJ => {
                sign_bit = input2 & 0x80000000;
                0.0
            }
            FpuAluOp::SGNJN => {
                sign_bit = !(input2 | 0x7fffffff);
                0.0
            }
            FpuAluOp::SGNJX => {
                sign_bit = (input1 ^ input2) & 0x80000000;
                0.0
            }
            FpuAluOp::Class => {
                if input1_f32.is_sign_negative() {
                    if input1_f32.is_infinite() {
                        input_mask = 0b1;
                    } else if input1_f32.is_normal() {
                        input_mask = 0b10;
                    } else if input1_f32.is_subnormal() {
                        input_mask = 0b100;
                    } else {
                        input_mask = 0b1000;
                    }
                } else if input1_f32.is_sign_positive() {
                    if input1_f32.is_infinite() {
                        input_mask = 0b10000000;
                    } else if input1_f32.is_normal() {
                        input_mask = 0b1000000;
                    } else if input1_f32.is_subnormal() {
                        input_mask = 0b100000;
                    } else {
                        input_mask = 0b10000;
                    }
                } else if input1_f32.is_nan() {
                    input_mask = 0b100000000;
                } else {
                    input_mask = 0b1000000000;
                }
                0.0
            }
            FpuAluOp::MAdd => input1_f32 * input2_f32 + input3_f32,
            FpuAluOp::MSub => input1_f32 * input2_f32 - input3_f32,
            FpuAluOp::NMSub => input1_f32.neg() * input2_f32 + input3_f32,
            FpuAluOp::NMAdd => input1_f32.neg() * input2_f32 - input3_f32,
            // No operation.
            FpuAluOp::Slt | FpuAluOp::Sle => 0.0,
        };

        if result_f32.is_nan() {
            self.state.alu_result = RISC_NAN as u64;
            return;
        }

        if (self.signals.fpu_alu_op == FpuAluOp::SGNJ)
            | (self.signals.fpu_alu_op == FpuAluOp::SGNJN)
            | (self.signals.fpu_alu_op == FpuAluOp::SGNJX)
        {
            self.state.alu_result = (input1_wo_sign | sign_bit) as i32 as u64;
            return;
        }

        if self.signals.fpu_alu_op == FpuAluOp::Class {
            self.state.alu_result = input_mask as u64;
            return;
        }

        self.state.alu_result = match self.signals.round_mode {
            RoundingMode::RNE => f32::to_bits(
                if (result_f32.ceil() - result_f32).abs() == (result_f32 - result_f32.floor()).abs()
                {
                    if result_f32.ceil() % 2.0 == 0.0 {
                        result_f32.ceil()
                    } else {
                        result_f32.floor()
                    }
                } else {
                    result_f32.round()
                },
            ),
            RoundingMode::RTZ => f32::to_bits(result_f32.trunc()),
            RoundingMode::RDN => f32::to_bits(result_f32.floor()),
            RoundingMode::RUP => f32::to_bits(result_f32.ceil()),
            RoundingMode::RMM => f32::to_bits(result_f32.round()),
            _ => f32::to_bits(result_f32),
        } as i32 as u64;
    }

    /// Perform a comparison.
    fn comparator(&mut self) {
        let input1 = self.state.read_data_1 as u32;
        let input2 = self.state.read_data_2 as u32;
        let input1_f32 = f32::from_bits(input1);
        let input2_f32 = f32::from_bits(input2);

        self.state.comparator_result = match self.signals.fpu_alu_op {
            FpuAluOp::MultiplicationOrEqual => (input1_f32 == input2_f32) as u64,
            FpuAluOp::Slt => (input1_f32 < input2_f32) as u64,
            FpuAluOp::Sle => (input1_f32 <= input2_f32) as u64,
            _ => 0,
        }
    }

    /// Write to the `Data` register. This register is used to transfer data between
    /// the main processor and the coprocessor.
    fn write_data(&mut self) {
        if let DataWrite::NoWrite = self.signals.data_write {
            return;
        }

        self.data = match self.signals.data_src {
            DataSrc::FloatingPointUnitRS1 => self.state.read_data_1,
            DataSrc::FloatingPointUnitComp => self.state.comparator_result,
            DataSrc::FloatingPointUnitMask => self.state.alu_result,
            DataSrc::FloatingPointBits => self.state.read_data_1 as i32 as u64,
            _ => self.state.data_from_main_processor,
        };
    }

    /// Set the data line that goes from `Read Data 2` to the multiplexer in the main processor
    /// controlled by [`MemWriteSrc`](super::control_signals::MemWriteSrc).
    fn write_fp_register_to_memory(&mut self) {
        self.state.fp_register_to_memory = self.state.read_data_2;
    }

    // ======================= Memory (MEM) =======================
    /// Set the data line between the multiplexer after the `Data` register and the
    /// multiplexer in the main processor controlled by the [`DataWrite`] control signal.
    fn set_data_writeback(&mut self) {
        if let DataWrite::NoWrite = self.signals.data_write {
            return;
        }

        self.state.data_writeback = match self.signals.data_src {
            DataSrc::MainProcessorUnit => match self.state.rs2 {
                0 => f32::to_bits(self.data as i32 as f32) as i32 as u64,
                1 => f32::to_bits(self.data as u32 as f32) as i32 as u64,
                2 => f32::to_bits(self.data as i64 as f32) as i32 as u64,
                3 => f32::to_bits(self.data as f32) as u64,
                _ => {
                    self.error(&format!(
                        "Unsupported Register Width `{:?}`",
                        self.state.rs2
                    ));
                    0
                }
            },
            DataSrc::FloatingPointUnitRS1 => {
                let data_unrounded = f32::from_bits(self.data as u32);
                let data_rounded = match self.signals.round_mode {
                    RoundingMode::RNE => {
                        if (data_unrounded.ceil() - data_unrounded).abs()
                            == (data_unrounded - data_unrounded.floor()).abs()
                        {
                            if data_unrounded.ceil() % 2.0 == 0.0 {
                                data_unrounded.ceil()
                            } else {
                                data_unrounded.floor()
                            }
                        } else {
                            data_unrounded.round()
                        }
                    }
                    RoundingMode::RTZ => data_unrounded.trunc(),
                    RoundingMode::RDN => data_unrounded.floor(),
                    RoundingMode::RUP => data_unrounded.ceil(),
                    _ => data_unrounded.round(),
                };

                match self.state.rs2 {
                    0 => {
                        if (data_rounded <= (-(2_i64.pow(31))) as f32)
                            | (data_rounded == f32::NEG_INFINITY)
                        {
                            -(2_i64.pow(31)) as u64
                        } else if (data_rounded >= (2_i64.pow(31) - 1) as f32)
                            | (data_rounded == f32::INFINITY)
                            | (data_rounded.is_nan())
                        {
                            (2_i64.pow(31) - 1) as u64
                        } else {
                            data_rounded as i32 as u64
                        }
                    }
                    1 => {
                        if (data_rounded <= 0.0) | (data_rounded == f32::NEG_INFINITY) {
                            0
                        } else if (data_rounded >= (2_u64.pow(32) - 1) as f32)
                            | (data_rounded == f32::INFINITY)
                            | (data_rounded.is_nan())
                        {
                            2_u64.pow(32) - 1
                        } else {
                            data_rounded as i32 as u64
                        }
                    }
                    2 => {
                        if (data_rounded <= (-(2_i64.pow(63))) as f32)
                            | (data_rounded == f32::NEG_INFINITY)
                        {
                            -(2_i64.pow(63)) as u64
                        } else if (data_rounded >= (2_i64.pow(63) - 1) as f32)
                            | (data_rounded == f32::INFINITY)
                            | (data_rounded.is_nan())
                        {
                            (2_i64.pow(63) - 1) as u64
                        } else {
                            data_rounded as i32 as u64
                        }
                    }
                    3 => {
                        if (data_rounded <= 0.0) | (data_rounded == f32::NEG_INFINITY) {
                            0
                        } else if (data_rounded >= (0xffffffffffffffff_u64) as f32)
                            | (data_rounded == f32::INFINITY)
                            | (data_rounded.is_nan())
                        {
                            0xffffffffffffffff
                        } else {
                            data_rounded as i32 as u64
                        }
                    }
                    _ => {
                        self.error(&format!(
                            "Unsupported Register Width `{:?}`",
                            self.state.rs2
                        ));
                        0
                    }
                }
            }
            DataSrc::MainProcessorBits => self.data as i32 as u64,
            _ => self.data,
        }
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
            DataWrite::YesWrite => self.state.data_writeback,
        };
        self.state.register_write_data = match self.signals.fpu_mem_to_reg {
            FpuMemToReg::UseDataWrite => self.state.register_write_mux_to_mux,
            FpuMemToReg::UseMemory => self.state.fp_register_data_from_main_processor,
        };
        self.registers.fpr[self.state.destination] = self.state.register_write_data;
    }
}
