//! Implementation of a MIPS64 datapath.
//!
//! It is assumed that while moving through stages, only one
//! instruction will be active any any given point in time. Due to this,
//! we consider the datapath to be a "pseudo-single-cycle datapath."
//!
//! For the most part, this datapath is an implementation of MIPS64 Version 6.
//! (See below for exceptions.)
//!
//! # Differences Compared to MIPS64 Version 6
//!
//! It should be noted that this datapath chooses to diverge from the MIPS64
//! version 6 specification for the sake of simplicity in a few places:
//!
//! - There is no exception handling, including that for integer overflow. (See
//!   [`MipsDatapath::alu()`].)
//! - Only the `addi` and `daddi` instructions follow the proper MIPS specification
//!   in terms of integer overflow. That is, if there is an overflow, the general-purpose
//!   register will not be written to. `add` and `dadd` will continue to write on
//!   overflow. `sub`, `dsub` will likewise continue to write on underflow.
//! - 32-bit instructions are treated exclusively with 32 bits, and the upper 32
//!   bits stored in a register are completely ignored in any of these cases. For
//!   example, before an `add` instruction, it should be checked whether it is a
//!   sign-extended 32-bit value stored in a 64-bit register. Instead, the upper
//!   32 bits are ignored when being used for 32-bit instructions.
//! - Instead of implementing the `cmp.condn.fmt` instructions, this datapath implements
//!   the `c.cond.fmt` instructions from MIPS64 version 5.
//! - This datapath implements the `addi` instruction as it exists in MIPS64 version 5.
//!   This instruction was deprecated in MIPS64 version 6 to allow for the `beqzalc`,
//!   `bnezalc`, `beqc`, and `bovc` instructions.
//! - Unlike the MIPS64 version 6 specification for the `jal` instruction, `PC + 4` is
//!   stored in `GPR[31]`, *not* `PC + 8`, as there is no implementation of branch
//!   delay slots.
//! - The "load upper immediate" (`lui`) instruction is officially supported as of MIPS64
//!   version 5. In version 6, `lui` is an assembly idiom for "add upper immediate" (`aui`)
//!   with `rs` = 0. However, `aui` is not officially supported nor tested in this
//!   datapath.

use super::super::datapath::Datapath;
use super::constants::*;
use super::control_signals::{floating_point::*, *};
use super::datapath_signals::*;
use super::instruction::*;
use super::{coprocessor::MipsFpCoprocessor, memory::Memory, registers::GpRegisters};

/// An implementation of a datapath for the MIPS64 ISA.
#[derive(PartialEq, Clone)]
pub struct MipsDatapath {
    pub registers: GpRegisters,
    pub memory: Memory,
    pub coprocessor: MipsFpCoprocessor,

    pub instruction: Instruction,
    pub signals: ControlSignals,
    pub datapath_signals: DatapathSignals,
    pub state: DatapathState,

    /// The currently-active stage in the datapath.
    pub current_stage: Stage,
}

/// A collection of all the data lines and wires in the datapath.
#[derive(Clone, Default, PartialEq)]
pub struct DatapathState {
    /// *Data line.* The currently loaded instruction. Initialized after the
    /// Instruction Fetch stage.
    pub instruction: u32,
    pub rs: u32,
    pub rt: u32,
    pub rd: u32,
    pub shamt: u32,
    pub funct: u32,
    pub imm: u32,

    /// *Data line.* Data read from the register file based on the `rs`
    /// field of the instruction. Initialized after the Instruction
    /// Decode stage.
    pub read_data_1: u64,

    /// *Data line.* Data read from the register file based on the `rt`
    /// field of the instruction. Initialized after the Instruction
    /// Decode stage.
    pub read_data_2: u64,

    /// *Data line.* The instruction's immediate value sign-extended to
    /// 64 bits. Initialized after the Instruction Decode stage.
    pub sign_extend: u64,

    /// *Data line.* The final result as provided by the ALU.
    /// Initialized after the Execute stage.
    pub alu_result: u64,

    /// *Data line.* The data retrieved from memory. Initialized after
    /// the Memory stage.
    pub memory_data: u64,

    /// *Data line.* The data after the `MemToReg` multiplexer, but
    /// before the `DataWrite` multiplexer in the main processor.
    pub data_result: u64,

    /// *Data line.* The data after the `DataWrite` multiplexer in the main
    /// processor and the main processor register file.
    pub register_write_data: u64,

    /// *Jump 26 bit line.* The low 26 bits of the instruction reserved
    /// for possible use by a J-type instruction.
    pub lower_26: u32,

    /// *Lower 26 << 2 line.* This line carries the low 28 bits of the
    /// jump address.
    pub lower_26_shifted_left_by_2: u32,

    /// *Jump address line.* This lines carries the concatenation of
    /// the high 36 bits of the PC, and `lower_26_shifted_left_by_2`.
    pub jump_address: u64,

    /// *Data line.* Contains PC + 4.
    pub pc_plus_4: u64,

    /// *Data line.* New PC value used if branching is set for an instruction.
    pub relative_pc_branch: u64,

    /// *Data line.* Determines the next value of the PC, given that the
    /// current instruction is not a jump.
    pub mem_mux1_to_mem_mux2: u64,

    /// *New PC line.* In the WB stage, this line is written to the PC.
    pub new_pc: u64,
}

/// The possible stages the datapath could be in during execution.
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum Stage {
    #[default]
    InstructionFetch,
    InstructionDecode,
    Execute,
    Memory,
    WriteBack,
}

impl Stage {
    /// Given a stage, return the next consecutive stage. If the last
    /// stage is given, return the first stage.
    fn get_next_stage(current_stage: Stage) -> Stage {
        match current_stage {
            Stage::InstructionFetch => Stage::InstructionDecode,
            Stage::InstructionDecode => Stage::Execute,
            Stage::Execute => Stage::Memory,
            Stage::Memory => Stage::WriteBack,
            Stage::WriteBack => Stage::InstructionFetch,
        }
    }
}

/// Handle an otherwise irrecoverable error within the datapath. At
/// present, this is the equivalent of `panic!()`.
pub fn error(message: &str) {
    panic!("{}", message);
}

impl Default for MipsDatapath {
    fn default() -> Self {
        let mut datapath = MipsDatapath {
            registers: GpRegisters::default(),
            memory: Memory::default(),
            coprocessor: MipsFpCoprocessor::default(),
            instruction: Instruction::default(),
            signals: ControlSignals::default(),
            datapath_signals: DatapathSignals::default(),
            state: DatapathState::default(),
            current_stage: Stage::default(),
        };

        // Set the stack pointer ($sp) to initially start at the end
        // of memory.
        datapath.registers.gpr[29] = super::memory::CAPACITY_BYTES as u64;

        datapath
    }
}

impl Datapath for MipsDatapath {
    type RegisterData = u64;
    type RegisterEnum = super::registers::GpRegisterType;
    type MemoryType = Memory;

    fn execute_instruction(&mut self) {
        // If the last instruction has not finished, finish it instead.
        if self.current_stage != Stage::InstructionFetch {
            self.finish_instruction();
            return;
        }

        // IF
        self.stage_instruction_fetch();

        // ID
        self.stage_instruction_decode();

        // EX
        self.stage_execute();

        // MEM
        self.stage_memory();

        // WB
        self.stage_writeback();
    }

    fn execute_stage(&mut self) {
        match self.current_stage {
            Stage::InstructionFetch => self.stage_instruction_fetch(),
            Stage::InstructionDecode => self.stage_instruction_decode(),
            Stage::Execute => self.stage_execute(),
            Stage::Memory => self.stage_memory(),
            Stage::WriteBack => self.stage_writeback(),
        }

        self.current_stage = Stage::get_next_stage(self.current_stage);
    }

    fn get_register_by_enum(&self, register: Self::RegisterEnum) -> u64 {
        self.registers[register]
    }

    fn get_memory(&self) -> &Self::MemoryType {
        &self.memory
    }

    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl MipsDatapath {
    /// Load a vector of 32-bit instructions into memory. If the process fails,
    /// from a lack of space or otherwise, an Err is returned.
    pub fn load_instructions(&mut self, instructions: Vec<u32>) -> Result<(), String> {
        for (i, data) in instructions.iter().enumerate() {
            self.memory.store_word((i as u64) * 4, *data)?
        }

        Ok(())
    }

    /// Finish the current instruction within the datapath. If the
    /// current stage is the first stage, do nothing as there is
    /// nothing to finish, only to start. (Use [`execute_instruction()`][MipsDatapath::execute_instruction()]
    /// in this case.)
    fn finish_instruction(&mut self) {
        while self.current_stage != Stage::InstructionFetch {
            self.execute_stage();
        }
    }

    /// Stage 1 of 5: Instruction Fetch (IF)
    ///
    /// Fetch the current instruction based on the given PC and load it
    /// into the datapath.
    fn stage_instruction_fetch(&mut self) {
        self.instruction_fetch();

        // Upper part of datapath, PC calculation
        self.pc_plus_4();

        self.coprocessor.set_instruction(self.state.instruction);
    }

    /// Stage 2 of 5: Instruction Decode (ID)
    ///
    /// Parse the instruction, set control signals, and read registers.
    fn stage_instruction_decode(&mut self) {
        self.instruction_decode();
        self.sign_extend();
        self.set_control_signals();
        self.read_registers();
        self.set_alu_control();

        // Upper part of datapath, PC calculation
        self.shift_lower_26_left_by_2();
        self.construct_jump_address();

        self.coprocessor.stage_instruction_decode();
        self.coprocessor
            .set_data_from_main_processor(self.state.read_data_2);
    }

    /// Stage 3 of 5: Execute (EX)
    ///
    /// Execute the current instruction with some arithmetic operation.
    fn stage_execute(&mut self) {
        self.alu();
        self.calc_relative_pc_branch();
        self.calc_cpu_branch_signal();
        self.coprocessor.stage_execute();
    }

    fn calc_relative_pc_branch(&mut self) {
        self.state.relative_pc_branch =
            (self.state.sign_extend << 2).wrapping_add(self.state.pc_plus_4);
    }

    // if Branch::YesBranch && AluZ::YesZero
    fn calc_cpu_branch_signal(&mut self) {
        // Yes I know this code looks truely garbage, please fix it
        // Rust does not allow the obvious clean option...
        let mut yes_branch: bool = false;
        if let Branch::YesBranch = self.signals.branch {
            yes_branch = true;
        }

        let mut beq: bool = false;
        if let BranchType::OnEqual = self.signals.branch_type {
            beq = true;
        }

        let mut bne: bool = false;
        if let BranchType::OnNotEqual = self.signals.branch_type {
            bne = true;
        }

        let mut z: bool = false;
        if let AluZ::YesZero = self.datapath_signals.alu_z {
            z = true;
        }

        if (yes_branch & z & beq) | (yes_branch & !z & bne) {
            self.datapath_signals.cpu_branch = CpuBranch::YesBranch;
        }
    }

    /// Stage 4 of 5: Memory (MEM)
    ///
    /// Read or write to memory.
    fn stage_memory(&mut self) {
        if let MemRead::YesRead = self.signals.mem_read {
            self.memory_read();
        }

        if let MemWrite::YesWrite = self.signals.mem_write {
            self.memory_write();
        }

        // Determine what data will be sent to the registers: either
        // the result from the ALU, or data retrieved from memory.
        self.state.data_result = match self.signals.mem_to_reg {
            MemToReg::UseAlu => self.state.alu_result,
            MemToReg::UseMemory => self.state.memory_data,
            MemToReg::UsePcPlusFour => self.state.pc_plus_4,
        };

        // PC calculation stuff from upper part of datapath
        self.calc_general_branch_signal();
        self.pick_pc_plus_4_or_relative_branch_addr_mux1();
        self.set_new_pc_mux2();

        self.coprocessor.stage_memory();
    }

    fn calc_general_branch_signal(&mut self) {
        if let CpuBranch::YesBranch = self.datapath_signals.cpu_branch {
            self.datapath_signals.general_branch = GeneralBranch::YesBranch;
            return;
        }

        if let FpuBranch::YesBranch = self.coprocessor.signals.fpu_branch {
            self.datapath_signals.general_branch = GeneralBranch::YesBranch;
        }
    }

    fn pick_pc_plus_4_or_relative_branch_addr_mux1(&mut self) {
        if let GeneralBranch::YesBranch = self.datapath_signals.general_branch {
            self.state.mem_mux1_to_mem_mux2 = self.state.relative_pc_branch;
        } else {
            self.state.mem_mux1_to_mem_mux2 = self.state.pc_plus_4;
        }
    }

    fn set_new_pc_mux2(&mut self) {
        self.state.new_pc = match self.signals.jump {
            Jump::NoJump => self.state.mem_mux1_to_mem_mux2,
            Jump::YesJump => self.state.jump_address,
        };
    }

    /// Stage 5 of 5: Writeback (WB)
    ///
    /// Write the result of the instruction's operation to a register,
    /// if desired. Additionally, set the PC for the next instruction.
    fn stage_writeback(&mut self) {
        self.coprocessor
            .set_fp_register_data_from_main_processor(self.state.data_result);
        self.register_write();
        self.set_pc();
        self.coprocessor.stage_writeback();
    }

    /// Load the raw binary instruction from memory and into the
    /// datapath. If there is an error with loading the word, assume
    /// the instruction to be bitwise zero and error.
    fn instruction_fetch(&mut self) {
        self.state.instruction = match self.memory.load_word(self.registers.pc) {
            Ok(data) => data,
            Err(e) => {
                error(e.as_str());
                0
            }
        }
    }

    fn pc_plus_4(&mut self) {
        self.state.pc_plus_4 = self.registers.pc + 4;
    }

    //    fn grab_lower_26_of_op(&mut self) {
    //         self.state.lower_26 = self.state.instruction & 0x03ffffff;
    //    }

    /// Decode an instruction into its individual fields.
    fn instruction_decode(&mut self) {
        self.instruction = Instruction::from(self.state.instruction);

        // Set the data lines based on the contents of the instruction.
        // Some lines will hold uninitialized values as a result.
        match self.instruction {
            Instruction::RType(r) => {
                self.state.rs = r.rs as u32;
                self.state.rt = r.rt as u32;
                self.state.rd = r.rd as u32;

                self.state.shamt = r.shamt as u32;
                self.state.funct = r.funct as u32;
            }
            Instruction::IType(i) => {
                self.state.rs = i.rs as u32;
                self.state.rt = i.rt as u32;
                self.state.rd = 0; // Placeholder
                self.state.imm = i.immediate as u32;
            }
            Instruction::FpuRegImmType(i) => {
                self.state.rs = 0; // Not applicable wire
                self.state.rt = i.rt as u32;
                self.state.rd = 0; // Not applicable
                self.state.imm = 0; // Not applicable
            }
            // R-type and comparison FPU instructions exclusively use the
            // FPU, so these data lines do not need to be used.
            Instruction::FpuRType(_) | Instruction::FpuCompareType(_) => (),
            Instruction::FpuIType(i) => {
                self.state.rs = i.base as u32;
                self.state.imm = i.offset as u32;
            }
            Instruction::JType(i) => {
                // self.state.lower_26 = self.state.instruction & 0x03ffffff;
                self.state.lower_26 = i.addr;
            } // _ => unimplemented!(),
        }
    }

    fn shift_lower_26_left_by_2(&mut self) {
        self.state.lower_26_shifted_left_by_2 = self.state.lower_26 << 2;
    }

    fn construct_jump_address(&mut self) {
        self.state.jump_address = (self.state.pc_plus_4 & 0xffff_ffff_f000_0000)
            | self.state.lower_26_shifted_left_by_2 as u64;
    }

    /// Extend the sign of a 16-bit value to the other 48 bits of a
    /// 64-bit value.
    fn sign_extend(&mut self) {
        self.state.sign_extend = ((self.state.imm as i16) as i64) as u64;
    }

    /// Set rtype control signals. This function may have a Match statement added
    /// in the future for dealing with different special case rtype instructions.
    fn set_rtype_control_signals(&mut self, r: RType) {
        self.signals.alu_op = AluOp::UseFunctField;
        self.signals.alu_src = AluSrc::ReadRegister2;
        self.signals.branch = Branch::NoBranch;
        self.signals.imm_shift = ImmShift::Shift0;
        self.signals.jump = Jump::NoJump;
        self.signals.mem_read = MemRead::NoRead;
        self.signals.mem_to_reg = MemToReg::UseAlu;
        self.signals.mem_write = MemWrite::NoWrite;
        self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
        self.signals.reg_dst = RegDst::Reg3;
        self.signals.reg_write = RegWrite::YesWrite;

        // The RegWidth signal might differ depending on the
        // specific R-type instruction.
        match reg_width_by_funct(r.funct) {
            Some(width) => self.signals.reg_width = width,
            None => unimplemented!(
                "funct code `{}` is unsupported for this opcode ({})",
                r.funct,
                r.op
            ),
        }
    }

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an I-type.
    fn set_itype_control_signals(&mut self, i: IType) {
        match i.op {
            // Register-immediate instructions are further defined
            // by the "rt" field.
            OPCODE_REGIMM => match i.rt {
                RMSUB_DAHI => {
                    self.signals = ControlSignals {
                        alu_op: AluOp::Addition,
                        alu_src: AluSrc::SignExtendedImmediate,
                        branch: Branch::NoBranch,
                        imm_shift: ImmShift::Shift32,
                        jump: Jump::NoJump,
                        mem_read: MemRead::NoRead,
                        mem_to_reg: MemToReg::UseAlu,
                        mem_write: MemWrite::NoWrite,
                        reg_dst: RegDst::Reg1,
                        reg_width: RegWidth::DoubleWord,
                        reg_write: RegWrite::YesWrite,
                        overflow_write_block: OverflowWriteBlock::NoBlock,
                        ..Default::default()
                    }
                }
                RMSUB_DATI => {
                    self.signals = ControlSignals {
                        alu_op: AluOp::Addition,
                        alu_src: AluSrc::SignExtendedImmediate,
                        branch: Branch::NoBranch,
                        imm_shift: ImmShift::Shift48,
                        jump: Jump::NoJump,
                        mem_read: MemRead::NoRead,
                        mem_to_reg: MemToReg::UseAlu,
                        mem_write: MemWrite::NoWrite,
                        reg_dst: RegDst::Reg1,
                        reg_width: RegWidth::DoubleWord,
                        reg_write: RegWrite::YesWrite,
                        overflow_write_block: OverflowWriteBlock::NoBlock,
                        ..Default::default()
                    }
                }
                _ => unimplemented!("rt field value `{}` for I-type opcode {}", i.rt, i.op),
            },

            OPCODE_ORI => {
                self.signals.alu_op = AluOp::Or;
                self.signals.alu_src = AluSrc::ZeroExtendedImmediate;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_LUI => {
                self.signals.alu_op = AluOp::LeftShift16;
                self.signals.alu_src = AluSrc::SignExtendedImmediate; // may  be fishy
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::Word;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_LW => {
                self.signals.alu_op = AluOp::Addition;
                self.signals.alu_src = AluSrc::SignExtendedImmediate; // may  be fishy
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::YesRead;
                self.signals.mem_to_reg = MemToReg::UseMemory;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::Word;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_SW => {
                self.signals.alu_op = AluOp::Addition;
                self.signals.alu_src = AluSrc::SignExtendedImmediate; // may  be fishy
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseMemory; // don't care
                self.signals.mem_write = MemWrite::YesWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::Word;
                self.signals.reg_write = RegWrite::NoWrite;
            }

            OPCODE_ANDI => {
                self.signals.alu_op = AluOp::And;
                self.signals.alu_src = AluSrc::ZeroExtendedImmediate;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_ADDI => {
                self.signals.alu_op = AluOp::AddWithNoWriteOnOverflow;
                self.signals.alu_src = AluSrc::SignExtendedImmediate;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::Word;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_ADDIU => {
                self.signals.alu_op = AluOp::Addition;
                self.signals.alu_src = AluSrc::SignExtendedImmediate;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::Word;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_DADDI => {
                self.signals.alu_op = AluOp::AddWithNoWriteOnOverflow;
                self.signals.alu_src = AluSrc::SignExtendedImmediate;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_DADDIU => {
                self.signals.alu_op = AluOp::Addition;
                self.signals.alu_src = AluSrc::SignExtendedImmediate;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::YesWrite;
            }

            OPCODE_BEQ => {
                self.signals.alu_op = AluOp::Subtraction;
                self.signals.alu_src = AluSrc::ReadRegister2;
                self.signals.branch = Branch::YesBranch;
                self.signals.branch_type = BranchType::OnEqual;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu; // don't care
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit; // don't care
                self.signals.reg_dst = RegDst::Reg2; // don't care
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::NoWrite;
            }

            OPCODE_BNE => {
                self.signals.alu_op = AluOp::Subtraction;
                self.signals.alu_src = AluSrc::ReadRegister2;
                self.signals.branch = Branch::YesBranch;
                self.signals.branch_type = BranchType::OnNotEqual;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu; // don't care
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit; // don't care
                self.signals.reg_dst = RegDst::Reg2; // don't care
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::NoWrite;
            }

            _ => unimplemented!("I-type instruction with opcode `{}`", i.op),
        }
    }

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an FPU I-type.
    fn set_fpu_itype_control_signals(&mut self, i: FpuIType) {
        match i.op {
            OPCODE_SWC1 => {
                self.signals = ControlSignals {
                    alu_op: AluOp::Addition,
                    alu_src: AluSrc::SignExtendedImmediate,
                    branch: Branch::NoBranch,
                    imm_shift: ImmShift::Shift0,
                    jump: Jump::NoJump,
                    mem_read: MemRead::NoRead,
                    mem_write: MemWrite::YesWrite,
                    mem_write_src: MemWriteSrc::FloatingPointUnit,
                    reg_width: RegWidth::Word,
                    reg_write: RegWrite::NoWrite,
                    ..Default::default()
                }
            }
            OPCODE_LWC1 => {
                self.signals = ControlSignals {
                    alu_op: AluOp::Addition,
                    alu_src: AluSrc::SignExtendedImmediate,
                    branch: Branch::NoBranch,
                    imm_shift: ImmShift::Shift0,
                    jump: Jump::NoJump,
                    mem_read: MemRead::YesRead,
                    mem_to_reg: MemToReg::UseMemory,
                    mem_write: MemWrite::NoWrite,
                    reg_width: RegWidth::Word,
                    reg_write: RegWrite::NoWrite,
                    ..Default::default()
                }
            }
            _ => unimplemented!("FPU I-type instruction with opcode `{}`", i.op),
        }
    }

    /// Set control signals for J-Type instructions
    fn set_jtype_control_signals(&mut self, j: JType) {
        match j.op {
            OPCODE_J => {
                self.signals.alu_op = AluOp::Addition;
                self.signals.alu_src = AluSrc::ReadRegister2;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::YesJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg2;
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::NoWrite;
            }
            OPCODE_JAL => {
                self.signals.alu_op = AluOp::Addition;
                self.signals.alu_src = AluSrc::ReadRegister2;
                self.signals.branch = Branch::NoBranch;
                self.signals.imm_shift = ImmShift::Shift0;
                self.signals.jump = Jump::YesJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UsePcPlusFour;
                self.signals.mem_write = MemWrite::YesWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::ReturnRegister;
                self.signals.reg_width = RegWidth::DoubleWord;
                self.signals.reg_write = RegWrite::YesWrite;
            }
            _ => unimplemented!("J-type instruction with opcode `{}`", j.op),
        };
    }

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an FPU register-immediate type.
    fn set_fpu_reg_imm_control_signals(&mut self, i: FpuRegImmType) {
        match i.sub {
            SUB_MT => {
                self.signals = ControlSignals {
                    branch: Branch::NoBranch,
                    jump: Jump::NoJump,
                    mem_read: MemRead::NoRead,
                    mem_write: MemWrite::NoWrite,
                    reg_width: RegWidth::Word,
                    reg_write: RegWrite::NoWrite,
                    overflow_write_block: OverflowWriteBlock::NoBlock,
                    ..Default::default()
                }
            }
            SUB_DMT => {
                self.signals = ControlSignals {
                    branch: Branch::NoBranch,
                    jump: Jump::NoJump,
                    mem_read: MemRead::NoRead,
                    mem_write: MemWrite::NoWrite,
                    reg_width: RegWidth::DoubleWord,
                    reg_write: RegWrite::NoWrite,
                    overflow_write_block: OverflowWriteBlock::NoBlock,
                    ..Default::default()
                }
            }
            SUB_MF => {
                self.signals = ControlSignals {
                    branch: Branch::NoBranch,
                    jump: Jump::NoJump,
                    mem_read: MemRead::NoRead,
                    mem_write: MemWrite::NoWrite,
                    reg_dst: RegDst::Reg2,
                    reg_width: RegWidth::Word,
                    reg_write: RegWrite::YesWrite,
                    overflow_write_block: OverflowWriteBlock::NoBlock,
                    ..Default::default()
                }
            }
            SUB_DMF => {
                self.signals = ControlSignals {
                    branch: Branch::NoBranch,
                    jump: Jump::NoJump,
                    mem_read: MemRead::NoRead,
                    mem_write: MemWrite::NoWrite,
                    reg_dst: RegDst::Reg2,
                    reg_width: RegWidth::DoubleWord,
                    reg_write: RegWrite::YesWrite,
                    overflow_write_block: OverflowWriteBlock::NoBlock,
                    ..Default::default()
                }
            }
            _ => unimplemented!(
                "FPU register-immediate instruction with sub code `{}`",
                i.sub
            ),
        }
    }

    /// Set the control signals for the datapath based on the
    /// instruction's opcode.
    fn set_control_signals(&mut self) {
        // Restore default behavior for this instruction, should OverflowWriteBlock
        // have been set from a previous instruction.
        self.signals.overflow_write_block = OverflowWriteBlock::NoBlock;

        match self.instruction {
            Instruction::RType(r) => {
                self.set_rtype_control_signals(r);
            }
            Instruction::IType(i) => {
                self.set_itype_control_signals(i);
            }
            Instruction::JType(j) => {
                self.set_jtype_control_signals(j);
            }
            Instruction::FpuRegImmType(i) => {
                self.set_fpu_reg_imm_control_signals(i);
            }
            // Main processor does nothing.
            Instruction::FpuRType(_) | Instruction::FpuCompareType(_) => {
                self.signals = ControlSignals {
                    branch: Branch::NoBranch,
                    jump: Jump::NoJump,
                    mem_read: MemRead::NoRead,
                    mem_write: MemWrite::NoWrite,
                    reg_write: RegWrite::NoWrite,
                    ..Default::default()
                };
            }
            Instruction::FpuIType(i) => {
                self.set_fpu_itype_control_signals(i);
            }
        }
    }

    /// Read the registers as specified from the instruction and pass
    /// the data into the datapath.
    fn read_registers(&mut self) {
        self.state.read_data_1 = self.registers.gpr[self.state.rs as usize];
        self.state.read_data_2 = self.registers.gpr[self.state.rt as usize];

        // Truncate the variable data if a 32-bit word is requested.
        if let RegWidth::Word = self.signals.reg_width {
            self.state.read_data_1 = self.state.read_data_1 as u32 as u64;
            self.state.read_data_2 = self.state.read_data_2 as u32 as u64;
        }
    }

    /// Set the ALU control signal based on the [`AluOp`] signal.
    fn set_alu_control(&mut self) {
        self.signals.alu_control = match self.signals.alu_op {
            AluOp::Addition => AluControl::Addition,
            AluOp::AddWithNoWriteOnOverflow => AluControl::AddWithNoWriteOnOverflow,
            AluOp::Subtraction => AluControl::Subtraction,
            AluOp::SetOnLessThanSigned => AluControl::SetOnLessThanSigned,
            AluOp::SetOnLessThanUnsigned => AluControl::SetOnLessThanUnsigned,
            AluOp::And => AluControl::And,
            AluOp::Or => AluControl::Or,
            AluOp::LeftShift16 => AluControl::LeftShift16,
            AluOp::UseFunctField => {
                match self.state.funct as u8 {
                    FUNCT_ADD | FUNCT_DADD => AluControl::Addition,
                    FUNCT_SUB | FUNCT_DSUB => AluControl::Subtraction,
                    FUNCT_AND => AluControl::And,
                    FUNCT_OR => AluControl::Or,
                    FUNCT_SLT => AluControl::SetOnLessThanSigned,
                    FUNCT_SLTU => AluControl::SetOnLessThanUnsigned,
                    FUNCT_SOP32 | FUNCT_SOP36 => match self.state.shamt as u8 {
                        ENC_DIV => AluControl::DivisionSigned,
                        _ => {
                            unimplemented!("MIPS Release 6 encoding `{}` unsupported for this function code ({})", self.state.shamt, self.state.funct);
                        }
                    },
                    FUNCT_SOP33 | FUNCT_SOP37 => match self.state.shamt as u8 {
                        ENC_DIVU => AluControl::DivisionUnsigned,
                        _ => {
                            unimplemented!("MIPS Release 6 encoding `{}` unsupported for this function code ({})", self.state.shamt, self.state.funct);
                        }
                    },
                    FUNCT_SOP30 | FUNCT_SOP34 => match self.state.shamt as u8 {
                        ENC_MUL => AluControl::MultiplicationSigned,
                        _ => {
                            unimplemented!("MIPS Release 6 encoding `{}` unsupported for this function code ({})", self.state.shamt, self.state.funct);
                        }
                    },
                    FUNCT_SOP31 | FUNCT_SOP35 => match self.state.shamt as u8 {
                        ENC_MULU => AluControl::MultiplicationUnsigned,
                        _ => {
                            unimplemented!("MIPS Release 6 encoding `{}` unsupported for this function code ({})", self.state.shamt, self.state.funct);
                        }
                    },
                    _ => {
                        unimplemented!("funct code `{}` is unsupported on ALU", self.state.funct);
                    }
                }
            }
        };
    }

    /// Perform an ALU operation.
    ///
    /// **Implementation Note:** Unlike the MIPS64 specification, this ALU
    /// does not handle exceptions due to integer overflow. However, it will
    /// set the [`OverflowWriteBlock`] signal on overflow when the operation
    /// [`AluControl`] is set to `AddWithNoWriteOverflow`.
    fn alu(&mut self) {
        // Left shift the immediate value based on the ImmShift control signal.
        let alu_immediate = match self.signals.imm_shift {
            ImmShift::Shift0 => self.state.sign_extend,
            ImmShift::Shift16 => self.state.sign_extend << 16,
            ImmShift::Shift32 => self.state.sign_extend << 32,
            ImmShift::Shift48 => self.state.sign_extend << 48,
        };

        // Specify the inputs for the operation. The first will always
        // be the first register, but the second may be either the
        // second register, the sign-extended immediate value, or the
        // zero-extended immediate value.
        let mut input1 = self.state.read_data_1;
        let mut input2 = match self.signals.alu_src {
            AluSrc::ReadRegister2 => self.state.read_data_2,
            AluSrc::SignExtendedImmediate => alu_immediate,
            AluSrc::ZeroExtendedImmediate => self.state.imm as u64,
        };

        // Truncate the inputs if 32-bit operations are expected.
        if let RegWidth::Word = self.signals.reg_width {
            input1 = input1 as i32 as u64;
            input2 = input2 as i32 as u64;
        }

        // Set the result.
        self.state.alu_result = match self.signals.alu_control {
            AluControl::Addition => input1.wrapping_add(input2),
            AluControl::AddWithNoWriteOnOverflow => {
                if let RegWidth::Word = self.signals.reg_width {
                    let input1 = input1 as u32;
                    let input2 = input2 as u32;
                    let sum = input1.overflowing_add(input2);
                    if sum.1 {
                        self.signals.overflow_write_block = OverflowWriteBlock::YesBlock;
                    }
                    sum.0 as u64
                } else {
                    let sum = input1.overflowing_add(input2);
                    if sum.1 {
                        self.signals.overflow_write_block = OverflowWriteBlock::YesBlock;
                    }
                    sum.0
                }
            }
            AluControl::Subtraction => (input1 as i64).wrapping_sub(input2 as i64) as u64,
            AluControl::SetOnLessThanSigned => ((input1 as i64) < (input2 as i64)) as u64,
            AluControl::SetOnLessThanUnsigned => (input1 < input2) as u64,
            AluControl::And => input1 & input2,
            AluControl::Or => input1 | input2,
            AluControl::LeftShift16 => input2 << 16,
            AluControl::Not => !input1,
            AluControl::MultiplicationSigned => ((input1 as i128) * (input2 as i128)) as u64,
            AluControl::MultiplicationUnsigned => ((input1 as u128) * (input2 as u128)) as u64,
            AluControl::DivisionSigned => {
                if input2 == 0 {
                    0
                } else {
                    ((input1 as i64) / (input2 as i64)) as u64
                }
            }
            AluControl::DivisionUnsigned => {
                if input2 == 0 {
                    0
                } else {
                    input1 / input2
                }
            }
        };

        // Truncate and sign-extend the output if 32-bit operations are expected.
        if let RegWidth::Word = self.signals.reg_width {
            self.state.alu_result = self.state.alu_result as i32 as i64 as u64;
        }

        // Set the zero bit/signal.
        if self.state.alu_result == 0 {
            self.datapath_signals.alu_z = AluZ::YesZero;
        }
    }

    /// Read from memory based on the address provided by the ALU in
    /// [`DatapathState::alu_result`]. Returns the result to [`DatapathState::memory_data`].
    /// Should the address be invalid or otherwise memory cannot be
    /// read at the given address, bitwise 0 will be used in lieu of
    /// any data.
    fn memory_read(&mut self) {
        let address = self.state.alu_result;

        // Load memory, first choosing the correct load function by the
        // RegWidth control signal, then reading the result from this
        // memory access.
        self.state.memory_data = match self.signals.reg_width {
            RegWidth::Word => self.memory.load_word(address).unwrap_or(0) as u64,
            RegWidth::DoubleWord => self.memory.load_double_word(address).unwrap_or(0),
        };
    }

    /// Write to memory based on the address provided by the ALU in
    /// [`DatapathState::alu_result`]. The source of the data being written to
    /// memory is determined by [`MemWriteSrc`].
    fn memory_write(&mut self) {
        let address = self.state.alu_result;

        let write_data = match self.signals.mem_write_src {
            MemWriteSrc::PrimaryUnit => self.state.read_data_2,
            MemWriteSrc::FloatingPointUnit => self.coprocessor.get_fp_register_to_memory(),
        };

        // Choose the correct store function based on the RegWidth
        // control signal.
        match self.signals.reg_width {
            RegWidth::Word => {
                self.memory.store_word(address, write_data as u32).ok();
            }
            RegWidth::DoubleWord => {
                self.memory.store_double_word(address, write_data).ok();
            }
        };
    }

    /// Write to a register. This will only write if the RegWrite
    /// control signal is set.
    fn register_write(&mut self) {
        // Determine what data will be sent to the register: either
        // the result from the ALU, or data retrieved from memory.
        self.state.data_result = match self.signals.mem_to_reg {
            MemToReg::UseAlu => self.state.alu_result,
            MemToReg::UseMemory => self.state.memory_data,
            MemToReg::UsePcPlusFour => self.state.pc_plus_4,
        };

        // Decide to retrieve data either from the main processor or the coprocessor.
        self.state.register_write_data = match self.coprocessor.signals.data_write {
            DataWrite::NoWrite => self.state.data_result,
            DataWrite::YesWrite => self.coprocessor.get_data_register(),
        };

        // Abort if the RegWrite signal is not set, or if the OverflowWriteBlock signal
        // is set and overriding write behavior.
        if self.signals.reg_write == RegWrite::NoWrite
            || self.signals.overflow_write_block == OverflowWriteBlock::YesBlock
        {
            return;
        }

        // Determine the destination for the data to write. This is
        // determined by the RegDst control signal.
        let destination = match self.signals.reg_dst {
            RegDst::Reg1 => self.state.rs as usize,
            RegDst::Reg2 => self.state.rt as usize,
            RegDst::Reg3 => self.state.rd as usize,
            RegDst::ReturnRegister => 31_usize,
        };

        // If we are attempting to write to register $zero, stop.
        if destination == 0 {
            return;
        }

        // If a 32-bit word is requested, ensure data is truncated and sign-extended.
        if let RegWidth::Word = self.signals.reg_width {
            self.state.data_result = self.state.data_result as i32 as u64;
        }

        // Write.
        self.registers.gpr[destination] = self.state.register_write_data;
    }

    /// Update the program counter register.
    ///
    /// This function is called from the WB stage.
    fn set_pc(&mut self) {
        self.registers.pc = self.state.new_pc;
    }
}
