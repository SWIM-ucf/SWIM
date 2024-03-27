//! Implementation of a RISC-V datapath.
//!
//! It is assumed that while moving through stages, only one
//! instruction will be active any any given point in time. Due to this,
//! we consider the datapath to be a "pseudo-single-cycle datapath."
//!
//! For the most part, this datapath is an implementation of RISC-V Spec 2.2 with extensions:
//! RV32I, RV64I, M, and F.
//! (See below for exceptions.)
//!
//! # Differences Compared to MIPS64 Version 6
//!
//! It should be noted that this datapath chooses to diverge from the MIPS64
//! version 6 specification for the sake of simplicity in a few places:
//!
//! - There is no exception handling, including that for integer overflow. (See
//!   [`MipsDatapath::alu()`] and the following bullet.)
//! - The `add`, `addi`, `dadd`, `daddi`, `sub`, and `dsub` instructions do not
//!   follow the proper MIPS specification in terms of integer overflow/wraparound.
//!   That is, if there is integer wraparound, the general-purpose register should
//!   not be written to. In our implementation, the general-purpose register is
//!   written to regardless.
//! - 32-bit instructions are treated exclusively with 32 bits, and the upper 32
//!   bits stored in a register are completely ignored in any of these cases. For
//!   example, before an `add` instruction, it should be checked whether it is a
//!   sign-extended 32-bit value stored in a 64-bit register. Instead, the upper
//!   32 bits are ignored when being used for 32-bit instructions.
//! - Instead of implementing the `cmp.condn.fmt` instructions, this datapath implements
//!   the `c.cond.fmt` instructions from MIPS64 version 5.
//! - Unlike MIPS specification, SWIM only uses 1 condition code register (`cc`), rather
//!   than offering 8 condition code registers. The datapath will assume that the `cc`
//!   field in a floating-point comparison or floating-point branch instruction is 0.
//! - This datapath implements the `addi` instruction as it exists in MIPS64 version 5.
//!   This instruction was deprecated in MIPS64 version 6 to allow for the `beqzalc`,
//!   `bnezalc`, `beqc`, and `bovc` instructions.
//! - This datapath implements `daddi` as it exists in MIPS64 version 5. This instruction
//!   was deprecated in MIPS64 version 6.
//! - Unlike the MIPS64 version 6 specification for the `jal` and `jalr` instructions,
//!   `PC + 4` is stored in `GPR[31]`, *not* `PC + 8`, as there is no implementation of
//!   branch delay slots.
//!
//! # Notes on `is_halted`
//!
//! - The datapath starts with the `is_halted` flag set.
//! - [`MipsDatapath::initialize()`] should be used to un-set `is_halted`.
//! - The `syscall` instruction simply performs a no-operation instruction, except for
//!   setting the boolean flag `is_halted`.
//! - Invalid instructions will cause the datapath to set the `is_halted` flag.

use super::super::datapath::Datapath;
use super::constants::*;
use super::control_signals::*;
use super::datapath_signals::*;
use super::instruction::*;
use super::registers::RiscGpRegisterType;
use super::{super::mips::memory::Memory, registers::RiscGpRegisters};
use crate::emulation_core::architectures::DatapathRef;
use crate::emulation_core::datapath::{DatapathUpdateSignal, Syscall};
use crate::emulation_core::riscv::registers::RiscGpRegisterType::{X10, X11};
use crate::emulation_core::stack::Stack;
use crate::emulation_core::stack::StackFrame;
use serde::{Deserialize, Serialize};

/// An implementation of a datapath for the MIPS64 ISA.
#[derive(Clone, PartialEq)]
pub struct RiscDatapath {
    pub registers: RiscGpRegisters,
    pub memory: Memory,
    pub stack: Stack,

    pub instruction: Instruction,
    pub signals: ControlSignals,
    pub datapath_signals: DatapathSignals,
    pub state: RiscDatapathState,

    /// The currently-active stage in the datapath.
    pub current_stage: RiscStage,

    /// Boolean value that states whether the datapath has halted.
    ///
    /// This is set in the event of any `syscall` instruction. To unset this,
    /// [`Self::initialize()`] should be used.
    is_halted: bool,
}

/// A collection of all the data lines and wires in the datapath.
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct RiscDatapathState {
    /// *Data line.* The currently loaded instruction. Initialized after the
    /// Instruction Fetch stage.
    pub instruction: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub rd: u32,
    pub shamt: u32,
    pub funct3: u32,
    pub funct7: u32,
    pub imm: u32,
    pub imm1: u32,
    pub imm2: u32,

    /// *Data line.* The first input of the ALU.
    pub alu_input1: u64,

    /// *Data line.* The second input of the ALU.
    pub alu_input2: u64,

    /// *Data line.* The final result as provided by the ALU.
    /// Initialized after the Execute stage.
    pub alu_result: u64,

    /// *Data line.* The data after the `MemToReg` multiplexer, but
    /// before the `DataWrite` multiplexer in the main processor.
    pub data_result: u64,

    // *Data line.* This line carries the idenfication number for the
    // register register-write will write to.
    pub write_register_destination: usize,

    /// *Jump address line.* This line carries the concatenation of
    /// the high 36 bits of the PC, and `lower_26_shifted_left_by_2`.
    pub jump_address: u64,

    /// *Jump 26 bit line.* The lower 26 bits of the instruction reserved
    /// for the location used by a J-type instruction.
    pub lower_26: u32,

    /// *Lower 26 << 2 line.* This line carries the lower 28 bits of the
    /// jump address.
    pub lower_26_shifted_left_by_2: u32,

    /// *Data line.* Determines the next value of the PC, given that the
    /// current instruction is not a jump.
    pub mem_mux1_to_mem_mux2: u64,

    /// *Data line.* The data retrieved from memory. Initialized after
    /// the Memory stage.
    pub memory_data: u64,

    /// *New PC line.* In the WB stage, this line is written to the PC.
    pub new_pc: u64,

    /// *Data line.* Contains PC + 4.
    pub pc_plus_4: u64,

    /// *Data line.* Data read from the register file based on the `rs1`
    /// field of the instruction. Initialized after the Instruction
    /// Decode stage.
    pub read_data_1: u64,

    /// *Data line.* Data read from the register file based on the `rd`
    /// field of the instruction. Initialized after the Instruction
    /// Decode stage.
    pub read_data_2: u64,

    /// *Data line.* The data after the `DataWrite` multiplexer in the main
    /// processor and the main processor register file.
    pub register_write_data: u64,

    /// *Data line.* New PC value used if branching is set for an instruction.
    pub relative_pc_branch: u64,

    /// *Data line.* The instruction's immediate value sign-extended to
    /// 64 bits. Initialized after the Instruction Decode stage.
    pub sign_extend: u64,

    /// *Data line.* The `sign_extend` line, shifted left by two bits.
    pub sign_extend_shift_left_by_2: u64,

    /// *Data line.* The data that will be written to memory.
    pub write_data: u64,
}

/// The possible stages the datapath could be in during execution.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum RiscStage {
    #[default]
    InstructionFetch,
    InstructionDecode,
    Execute,
    Memory,
    WriteBack,
}

impl RiscStage {
    /// Given a stage, return the next consecutive stage. If the last
    /// stage is given, return the first stage.
    fn get_next_stage(current_stage: RiscStage) -> RiscStage {
        match current_stage {
            RiscStage::InstructionFetch => RiscStage::InstructionDecode,
            RiscStage::InstructionDecode => RiscStage::Execute,
            RiscStage::Execute => RiscStage::Memory,
            RiscStage::Memory => RiscStage::WriteBack,
            RiscStage::WriteBack => RiscStage::InstructionFetch,
        }
    }
}

impl Default for RiscDatapath {
    fn default() -> Self {
        let mut datapath = RiscDatapath {
            registers: RiscGpRegisters::default(),
            memory: Memory::default(),
            stack: Stack::default(),
            instruction: Instruction::default(),
            signals: ControlSignals::default(),
            datapath_signals: DatapathSignals::default(),
            state: RiscDatapathState::default(),
            current_stage: RiscStage::default(),
            is_halted: true,
        };

        // Set the stack pointer ($sp) to initially start at the end
        // of memory.
        datapath.registers.gpr[29] = super::super::mips::memory::CAPACITY_BYTES as u64;

        datapath
    }
}

impl Datapath for RiscDatapath {
    type RegisterData = u64;

    /// Reset the datapath, load instructions into memory, and un-sets the `is_halted`
    /// flag. If the process fails, an [`Err`] is returned.
    fn initialize(&mut self, initial_pc: usize, instructions: Vec<u32>) -> Result<(), String> {
        self.reset();
        self.load_instructions(instructions)?;
        self.registers.pc = initial_pc as u64;
        self.is_halted = false;

        Ok(())
    }

    fn execute_instruction(&mut self) -> DatapathUpdateSignal {
        let mut result_signals = DatapathUpdateSignal::default();
        loop {
            // Stop early if the datapath has halted.
            if self.is_halted {
                break;
            }

            result_signals |= self.execute_stage();

            // This instruction is finished when the datapath has returned
            // to the IF stage.
            if self.current_stage == RiscStage::InstructionFetch {
                break;
            }
        }
        result_signals
    }

    fn execute_stage(&mut self) -> DatapathUpdateSignal {
        // If the datapath is halted, do nothing.
        if self.is_halted {
            return DatapathUpdateSignal::default();
        }

        let res = match self.current_stage {
            RiscStage::InstructionFetch => self.stage_instruction_fetch(),
            RiscStage::InstructionDecode => self.stage_instruction_decode(),
            RiscStage::Execute => self.stage_execute(),
            RiscStage::Memory => self.stage_memory(),
            RiscStage::WriteBack => self.stage_writeback(),
        };

        self.current_stage = RiscStage::get_next_stage(self.current_stage);
        res
    }

    fn set_register_by_str(&mut self, register: &str, data: Self::RegisterData) {
        let register = &mut self.registers[register];
        *register = data;
    }

    fn get_memory(&self) -> &Memory {
        &self.memory
    }

    fn set_memory(&mut self, _ptr: u64, _data: u32) {
        self.memory.store_word(_ptr, _data).unwrap_or_default();
    }

    fn is_halted(&self) -> bool {
        self.is_halted
    }

    fn reset(&mut self) {
        std::mem::take(self);
    }

    fn as_datapath_ref(&self) -> DatapathRef {
        DatapathRef::RISCV(self)
    }

    fn set_fp_register_by_str(&mut self, _register: &str, _data: Self::RegisterData) {
        todo!()
    }

    fn get_memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    fn halt(&mut self) {
        self.is_halted = true;
    }

    fn get_syscall_arguments(&self) -> Syscall {
        Syscall::from_register_data(
            self.registers[X10],
            self.registers[X11],
            0.0f32,
            0.0f64, // TODO: Add the appropriate arguments after F extension support
        )
    }
}

impl RiscDatapath {
    // ===================== General Functions =====================

    /// Load a vector of 32-bit instructions into memory. If the process fails,
    /// from a lack of space or otherwise, an [`Err`] is returned.
    fn load_instructions(&mut self, instructions: Vec<u32>) -> Result<(), String> {
        for (i, data) in instructions.iter().enumerate() {
            self.memory.store_word((i as u64) * 4, *data)?
        }

        Ok(())
    }

    /// Handle an otherwise irrecoverable error within the datapath.
    pub fn error(&mut self, _message: &str) {
        self.is_halted = true;
    }

    // ========================== Stages ==========================
    /// Stage 1 of 5: Instruction Fetch (IF)
    ///
    /// Fetch the current instruction based on the given PC and load it
    /// into the datapath.
    fn stage_instruction_fetch(&mut self) -> DatapathUpdateSignal {
        self.instruction_fetch();

        // Upper part of datapath, PC calculation
        self.pc_plus_4();

        // Both state and coprocessor state always update
        DatapathUpdateSignal {
            changed_state: true,
            ..Default::default()
        }
    }

    /// Stage 2 of 5: Instruction Decode (ID)
    ///
    /// Parse the instruction, set control signals, and read registers.
    ///
    /// If the instruction is determined to be a `syscall`, immediately
    /// finish the instruction and set the `is_halted` flag.
    fn stage_instruction_decode(&mut self) -> DatapathUpdateSignal {
        self.instruction_decode();
        self.set_control_signals();
        self.set_immediate();
        self.read_registers();

        // Check if we hit a syscall or breakpoint and signal it to the caller.
        let (hit_syscall, hit_breakpoint) = (
            self.signals.sys_op == SysOp::ECALL,
            self.signals.sys_op == SysOp::EBREAK,
        );

        // Instruction decode always involves a state update
        DatapathUpdateSignal {
            changed_state: true,
            hit_syscall,
            hit_breakpoint,
            ..Default::default()
        }
    }

    /// Stage 3 of 5: Execute (EX)
    ///
    /// Execute the current instruction with some arithmetic operation.
    fn stage_execute(&mut self) -> DatapathUpdateSignal {
        self.alu();
        self.calc_relative_pc_branch();
        self.calc_cpu_branch_signal();

        DatapathUpdateSignal {
            changed_state: true,
            ..Default::default()
        }
    }

    /// Stage 4 of 5: Memory (MEM)
    ///
    /// Read or write to memory.
    fn stage_memory(&mut self) -> DatapathUpdateSignal {
        match self.signals.read_write {
            ReadWrite::LoadByte => self.memory_read(),
            ReadWrite::LoadByteUnsigned => self.memory_read(),
            ReadWrite::LoadHalf => self.memory_read(),
            ReadWrite::LoadHalfUnsigned => self.memory_read(),
            ReadWrite::LoadWord => self.memory_read(),
            ReadWrite::LoadWordUnsigned => self.memory_read(),
            ReadWrite::LoadDouble => self.memory_read(),
            ReadWrite::NoLoadStore => (),
            ReadWrite::StoreByte => self.memory_write(),
            ReadWrite::StoreHalf => self.memory_write(),
            ReadWrite::StoreWord => self.memory_write(),
            ReadWrite::StoreDouble => self.memory_write(),
        }

        // PC calculation stuff from upper part of datapath
        self.calc_general_branch_signal();
        self.pick_pc_plus_4_or_relative_branch_addr_mux1();
        self.set_new_pc_mux2();

        let mut changed_stack = false;

        // If this is the first instruction, push the initial frame to the stack
        if self.stack.is_empty() {
            let frame = StackFrame::new(
                self.state.instruction,
                self.registers.pc,
                self.state.pc_plus_4,
                self.registers[RiscGpRegisterType::X2],
                // self.registers[GpRegisterType::Sp],
                self.registers.pc,
            );
            self.stack.push(frame);
            changed_stack = true;
        }

        let hit_jump = matches!(self.signals.branch_jump, BranchJump::J);
        if hit_jump {
            // Add current line to stack if we call a function
            let frame = StackFrame::new(
                self.state.instruction,
                self.registers.pc,
                self.state.pc_plus_4,
                self.registers[RiscGpRegisterType::X2],
                self.state.new_pc,
            );
            self.stack.push(frame);
            changed_stack = true;
        }

        DatapathUpdateSignal {
            changed_state: true,
            changed_memory: ((self.signals.read_write == ReadWrite::StoreByte)
                | (self.signals.read_write == ReadWrite::StoreDouble)
                | (self.signals.read_write == ReadWrite::StoreHalf)
                | (self.signals.read_write == ReadWrite::StoreWord)),
            changed_stack,
            ..Default::default()
        }
    }

    /// Stage 5 of 5: Writeback (WB)
    ///
    /// Write the result of the instruction's operation to a register,
    /// if desired. Additionally, set the PC for the next instruction.
    fn stage_writeback(&mut self) -> DatapathUpdateSignal {
        self.register_write();
        self.set_pc();

        // check if we are writing to the stack pointer
        let mut changed_stack = false;
        if self.state.write_register_destination == RiscGpRegisterType::X2 as usize {
            if let Some(last_frame) = self.stack.peek() {
                if self.state.register_write_data >= last_frame.frame_pointer {
                    // dellocating stack space
                    self.stack.pop();
                    changed_stack = true;
                }
            }
        }

        DatapathUpdateSignal {
            changed_state: true,
            changed_registers: true, // Always true because pc always gets updated
            changed_stack,
            ..Default::default()
        }
    }

    // ================== Instruction Fetch (IF) ==================
    /// Load the raw binary instruction from memory and into the
    /// datapath. If there is an error with loading the word, assume
    /// the instruction to be bitwise zero and error.
    fn instruction_fetch(&mut self) {
        self.state.instruction = match self.memory.load_word(self.registers.pc) {
            Ok(data) => data,
            Err(e) => {
                self.error(e.as_str());
                0
            }
        }
    }

    fn pc_plus_4(&mut self) {
        self.state.pc_plus_4 = self.registers.pc + 4;
    }

    // ================== Instruction Decode (ID) ==================
    /// Decode an instruction into its individual fields.
    fn instruction_decode(&mut self) {
        match Instruction::try_from(self.state.instruction) {
            Ok(instruction) => self.instruction = instruction,
            Err(message) => {
                self.error(&message);
                return;
            }
        }

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
            Instruction::BType(b) => {
                self.state.rs2 = b.rs2 as u32;
                self.state.rs1 = b.rs1 as u32;
                self.state.funct3 = b.funct3 as u32;
                self.state.imm1 = b.imm1 as u32;
                self.state.imm2 = b.imm2 as u32;
            }
            Instruction::UType(u) => {
                self.state.imm = u.imm;
                self.state.rd = u.rd as u32;
            }
            Instruction::JType(j) => {
                self.state.imm = j.imm;
                self.state.rd = j.rd as u32;
            }
            Instruction::R4Type(r) => {
                self.state.rd = r.rd as u32;
            }
        }
    }

    fn set_immediate(&mut self) {
        let mut signed_imm = 0x0000;
        if self.state.instruction >> 31 == 1 {
            signed_imm = 0xffffffff;
        }

        signed_imm = match self.signals.imm_select {
            ImmSelect::ISigned => (signed_imm << 12) | self.state.imm,
            ImmSelect::IShamt => (signed_imm << 12) | self.state.imm,
            ImmSelect::IUnsigned => self.state.imm,
            ImmSelect::SType => ((signed_imm << 7) | self.state.imm1) << 5 | self.state.imm2,
            ImmSelect::BType => {
                ((((signed_imm << 1) | (self.state.imm2 & 0x01)) << 6) | (self.state.imm1 & 0x3f))
                    << 5
                    | (self.state.imm2 & 0x1e)
            }
            ImmSelect::UType => ((signed_imm << 20) | self.state.imm) << 12,
            ImmSelect::JType => {
                (((((signed_imm << 8) | (self.state.imm & 0xff)) << 1)
                    | (self.state.imm >> 8 & 0x01))
                    << 11)
                    | (self.state.imm >> 8 & 0x7fe)
            }
        };

        self.state.imm = signed_imm;
    }

    /// Set the control signals for the datapath based on the
    /// instruction's opcode.
    fn set_control_signals(&mut self) {
        match self.instruction {
            Instruction::RType(r) => {
                self.set_rtype_control_signals(r);
            }
            Instruction::IType(i) => {
                self.set_itype_control_signals(i);
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

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an R-type.
    fn set_rtype_control_signals(&mut self, r: RType) {
        self.signals = ControlSignals {
            op2_select: OP2Select::DATA2,
            branch_jump: BranchJump::NoBranch,
            read_write: ReadWrite::NoLoadStore,
            wb_sel: WBSel::UseAlu,
            reg_dst: RegDst::Reg3,
            reg_write_en: RegWriteEn::YesWrite,
            ..Default::default()
        };

        if r.op == OPCODE_OP_32 {
            self.datapath_signals.reg_width = RegisterWidth::HalfWidth;
        }

        match r.funct3 {
            0 => match r.funct7 {
                0b0000000 => self.signals.alu_op = AluOp::Addition,
                0b0100000 => self.signals.alu_op = AluOp::Subtraction,
                _ => (),
            },
            1 => self.signals.alu_op = AluOp::ShiftLeftLogical(self.state.rs2),
            2 => self.signals.alu_op = AluOp::SetOnLessThanSigned,
            3 => self.signals.alu_op = AluOp::SetOnLessThanUnsigned,
            4 => self.signals.alu_op = AluOp::Xor,
            5 => match r.funct7 {
                0b0000000 => self.signals.alu_op = AluOp::ShiftRightLogical(self.state.rs2),
                0b0100000 => self.signals.alu_op = AluOp::ShiftRightArithmetic(self.state.rs2),
                _ => (),
            },
            6 => self.signals.alu_op = AluOp::Or,
            7 => self.signals.alu_op = AluOp::And,
            _ => (),
        }
    }

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an I-type.
    fn set_itype_control_signals(&mut self, i: IType) {
        self.signals = ControlSignals {
            reg_dst: RegDst::Reg3,
            reg_write_en: RegWriteEn::YesWrite,
            ..Default::default()
        };

        if i.op == OPCODE_IMM_32 {
            self.datapath_signals.reg_width = RegisterWidth::HalfWidth;
            if self.state.shamt >> 5 != 0 {
                self.error("Unsupported Instruction!");
            }
        }

        match i.op {
            OPCODE_IMM | OPCODE_IMM_32 => match i.funct3 {
                0 => self.signals.alu_op = AluOp::Addition,
                1 => self.signals.alu_op = AluOp::ShiftLeftLogical(self.state.shamt),

                2 => self.signals.alu_op = AluOp::SetOnLessThanSigned,
                3 => self.signals.alu_op = AluOp::SetOnLessThanUnsigned,
                4 => self.signals.alu_op = AluOp::Xor,
                5 => {
                    match i.imm >> 6 {
                        0b000000 => {
                            self.signals.alu_op = AluOp::ShiftRightLogical(self.state.shamt)
                        }
                        0b010000 => {
                            self.signals.alu_op = AluOp::ShiftRightArithmetic(self.state.shamt)
                        }
                        _ => (),
                    };
                }
                6 => self.signals.alu_op = AluOp::Or,
                7 => self.signals.alu_op = AluOp::And,
                _ => (),
            },
            OPCODE_JALR => {
                self.signals.imm_select = ImmSelect::IUnsigned;
                self.signals.wb_sel = WBSel::UsePcPlusFour;
                self.signals.branch_jump = BranchJump::J;
            }
            OPCODE_LOAD => {
                self.signals.wb_sel = WBSel::UseMemory;
                match i.funct3 {
                    0 => self.signals.read_write = ReadWrite::LoadByte,
                    1 => self.signals.read_write = ReadWrite::LoadHalf,
                    2 => self.signals.read_write = ReadWrite::LoadWord,
                    3 => self.signals.read_write = ReadWrite::LoadDouble,
                    4 => self.signals.read_write = ReadWrite::LoadByteUnsigned,
                    5 => self.signals.read_write = ReadWrite::LoadHalfUnsigned,
                    6 => self.signals.read_write = ReadWrite::LoadWordUnsigned,
                    _ => (),
                }
            }
            OPCODE_SYSTEM => {
                self.signals.imm_select = ImmSelect::IUnsigned;
                self.signals.wb_sel = WBSel::UseAlu;

                match i.funct3 {
                    0 => {
                        self.signals.sys_op = match i.imm {
                            0 => SysOp::ECALL,
                            1 => SysOp::EBREAK,
                            _ => SysOp::None,
                        };
                        self.signals.reg_write_en = RegWriteEn::NoWrite;
                    }
                    1 => self.signals.sys_op = SysOp::CSRReadWrite,
                    2 => self.signals.sys_op = SysOp::CSRReadSet,
                    3 => self.signals.sys_op = SysOp::CSRReadClear,
                    5 => {
                        self.signals.sys_op = SysOp::CSRReadWrite;
                        self.signals.op1_select = OP1Select::IMM;
                    }
                    6 => {
                        self.signals.sys_op = SysOp::CSRReadSet;
                        self.signals.op1_select = OP1Select::IMM;
                    }
                    7 => {
                        self.signals.sys_op = SysOp::CSRReadClear;
                        self.signals.op1_select = OP1Select::IMM;
                    }
                    _ => {
                        self.error("Unsupported Instruction!");
                    }
                }
            }
            _ => (),
        }
    }

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an S-type.
    fn set_stype_control_signals(&mut self, s: SType) {
        self.signals = ControlSignals {
            imm_select: ImmSelect::SType,
            reg_write_en: RegWriteEn::NoWrite,
            ..Default::default()
        };

        match s.funct3 {
            0 => self.signals.read_write = ReadWrite::StoreByte,
            1 => self.signals.read_write = ReadWrite::StoreHalf,
            2 => self.signals.read_write = ReadWrite::StoreWord,
            3 => self.signals.read_write = ReadWrite::StoreDouble,
            _ => (),
        }
    }

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an B-type.
    fn set_btype_control_signals(&mut self, b: BType) {
        self.signals = ControlSignals {
            imm_select: ImmSelect::BType,
            reg_write_en: RegWriteEn::NoWrite,
            ..Default::default()
        };

        match b.funct3 {
            0 => self.signals.branch_jump = BranchJump::Beq,
            1 => self.signals.branch_jump = BranchJump::Bne,
            4 => self.signals.branch_jump = BranchJump::Blt,
            5 => self.signals.branch_jump = BranchJump::Bge,
            6 => self.signals.branch_jump = BranchJump::Bltu,
            7 => self.signals.branch_jump = BranchJump::Bgeu,
            _ => (),
        }
    }

    /// Set the control signals for the datapath, specifically in the
    /// case where the instruction is an U-type.
    fn set_utype_control_signals(&mut self, u: UType) {
        self.signals = ControlSignals {
            imm_select: ImmSelect::UType,
            op1_select: OP1Select::PC,
            alu_op: AluOp::Addition,
            reg_dst: RegDst::Reg3,
            reg_write_en: RegWriteEn::YesWrite,
            ..Default::default()
        };

        match u.op {
            OPCODE_AUIPC => self.signals.wb_sel = WBSel::UseAlu,
            OPCODE_LUI => self.signals.wb_sel = WBSel::UseImmediate,
            _ => (),
        }
    }

    /// Set control signals for J-Type instructions
    fn set_jtype_control_signals(&mut self, _j: JType) {
        self.signals = ControlSignals {
            imm_select: ImmSelect::JType,
            branch_jump: BranchJump::J,
            wb_sel: WBSel::UsePcPlusFour,
            reg_write_en: RegWriteEn::YesWrite,
            ..Default::default()
        };
    }

    /// Read the registers as specified from the instruction and pass
    /// the data into the datapath.
    fn read_registers(&mut self) {
        self.state.read_data_1 = self.registers.gpr[self.state.rs1 as usize];
        self.state.read_data_2 = self.registers.gpr[self.state.rs2 as usize];
    }

    // ======================= Execute (EX) =======================
    /// Perform an ALU operation.
    ///
    /// **Implementation Note:** Unlike the MIPS64 specification, this ALU
    /// does not handle exceptions due to integer overflow.
    fn alu(&mut self) {
        // Specify the inputs for the operation. The first will always
        // be the first register, but the second may be either the
        // second register, the sign-extended immediate value, or the
        // zero-extended immediate value.
        self.state.alu_input1 = match self.signals.op1_select {
            OP1Select::PC => self.registers.pc,
            OP1Select::DATA1 => self.state.read_data_1,
            OP1Select::IMM => self.state.rs1 as u64,
        };

        self.state.alu_input2 = match self.signals.op2_select {
            OP2Select::DATA2 => self.state.read_data_2,
            OP2Select::IMM => self.state.imm as i64 as u64,
        };

        if self.signals.sys_op != SysOp::None {
            self.csr_handler();
            return;
        }

        if self.datapath_signals.reg_width == RegisterWidth::HalfWidth {
            self.state.alu_input1 = self.state.alu_input1 as u32 as i64 as u64;
            self.state.alu_input2 = self.state.alu_input2 as u32 as i64 as u64;
        }

        // Set the result.
        self.state.alu_result = match self.signals.alu_op {
            AluOp::Addition => self.state.alu_input1.wrapping_add(self.state.alu_input2),
            AluOp::Subtraction => {
                (self.state.alu_input1 as i64).wrapping_sub(self.state.alu_input2 as i64) as u64
            }
            AluOp::SetOnLessThanSigned => {
                ((self.state.alu_input1 as i64) < (self.state.alu_input2 as i64)) as u64
            }
            AluOp::SetOnLessThanUnsigned => (self.state.alu_input1 < self.state.alu_input2) as u64,
            AluOp::And => self.state.alu_input1 & self.state.alu_input2,
            AluOp::Or => self.state.alu_input1 | self.state.alu_input2,
            AluOp::Xor => self.state.alu_input1 ^ self.state.alu_input2,
            AluOp::ShiftLeftLogical(shamt) => self.state.alu_input1 << shamt,
            AluOp::ShiftRightLogical(shamt) => self.state.alu_input1 >> shamt,
            AluOp::ShiftRightArithmetic(shamt) => (self.state.alu_input1 as i64 >> shamt) as u64,
            AluOp::MultiplicationSigned => {
                ((self.state.alu_input1 as i128) * (self.state.alu_input2 as i128)) as u64
            }
            AluOp::MultiplicationUnsigned => {
                ((self.state.alu_input1 as u128) * (self.state.alu_input2 as u128)) as u64
            }
            AluOp::DivisionSigned => {
                if self.state.alu_input2 == 0 {
                    0
                } else {
                    ((self.state.alu_input1 as i64) / (self.state.alu_input2 as i64)) as u64
                }
            }
            AluOp::DivisionUnsigned => {
                if self.state.alu_input2 == 0 {
                    0
                } else {
                    self.state.alu_input1 / self.state.alu_input2
                }
            }
            _ => 0,
        };

        if self.datapath_signals.reg_width == RegisterWidth::HalfWidth {
            self.state.alu_result = self.state.alu_result as u32 as i64 as u64;
        }

        if self.signals.branch_jump == BranchJump::J {
            self.construct_jump_address();
        }

        // Set the zero bit/signal.
        self.datapath_signals.alu_z = match self.state.alu_result {
            0 => AluZ::YesZero,
            _ => AluZ::NoZero,
        };
    }

    fn csr_handler(&mut self) {
        match self.signals.sys_op {
            SysOp::CSRReadWrite => {
                if self.registers["X0"] != self.registers.gpr[self.state.rd as usize] {
                    self.state.alu_result = self.registers.gpr[self.state.imm as usize];
                } else {
                    self.state.alu_result = self.registers.gpr[self.state.rd as usize];
                }
                self.registers.gpr[self.state.imm as usize] = self.state.alu_input1;
            }
            SysOp::CSRReadSet => {
                self.state.alu_result = self.registers.gpr[self.state.imm as usize];
                if self.registers["X0"] != self.state.alu_input1 {
                    self.registers.gpr[self.state.imm as usize] |= self.state.alu_input1;
                }
            }
            SysOp::CSRReadClear => {
                self.state.alu_result = self.registers.gpr[self.state.imm as usize];
                if self.registers["X0"] != self.state.alu_input1 {
                    self.registers.gpr[self.state.imm as usize] &= !self.state.alu_input1;
                }
            }
            SysOp::None => self.error("Impossible/Unsupported Instruction!"),
            _ => self.state.alu_result = 0,
        }
    }

    fn construct_jump_address(&mut self) {
        self.state.rd = self.state.pc_plus_4 as u32;
        self.state.jump_address = match self.instruction {
            Instruction::IType(_i) => {
                (self.state.imm as u64 + self.state.read_data_1) & 0xfffffffffffffff0
            }
            Instruction::JType(_j) => self.state.imm as u64 + self.registers.pc,
            _ => self.state.jump_address,
        }
    }

    fn calc_relative_pc_branch(&mut self) {
        self.state.relative_pc_branch = ((self.state.imm & 0x00000fff) as u64) + self.registers.pc;
    }

    /// Determine the value of the [`CpuBranch`] signal.
    fn calc_cpu_branch_signal(&mut self) {
        // Start by assuming there is no branch.
        self.datapath_signals.cpu_branch = CpuBranch::NoBranch;

        // condition_is_true is based on the ALU and the BranchType. This
        // is the line between the multiplexer and the AND gate, where the
        // AND gate has as input the Branch control signal and said
        // multiplexer.
        //
        // Depending on the branch type, this may use the ALU's Zero signal
        // as-is or inverted.
        let condition_is_true = match self.signals.branch_jump {
            BranchJump::Beq => self.state.read_data_1 == self.state.read_data_2,
            BranchJump::Bne => self.state.read_data_1 != self.state.read_data_2,
            BranchJump::Bge => self.state.read_data_1 as i64 >= self.state.read_data_2 as i64,
            BranchJump::Bgeu => self.state.read_data_1 >= self.state.read_data_2,
            BranchJump::Blt => (self.state.read_data_1 as i64) < (self.state.read_data_2 as i64),
            BranchJump::Bltu => self.state.read_data_1 < self.state.read_data_2,
            _ => false,
        };

        if condition_is_true {
            self.datapath_signals.cpu_branch = CpuBranch::YesBranch;
        }
    }

    // ======================= Memory (MEM) =======================
    /// Read from memory based on the address provided by the ALU in
    /// [`RiscDatapathState::alu_result`]. Returns the result to [`RiscDatapathState::memory_data`].
    /// Should the address be invalid or otherwise memory cannot be
    /// read at the given address, bitwise 0 will be used in lieu of
    /// any data.
    fn memory_read(&mut self) {
        let address = self.state.alu_result;

        // Load memory, first choosing the correct load function by the
        // RegWidth control signal, then reading the result from this
        // memory access.
        self.state.memory_data = match self.signals.read_write {
            ReadWrite::LoadByte => self.memory.load_byte(address).unwrap_or(0) as i64 as u64,
            ReadWrite::LoadByteUnsigned => self.memory.load_byte(address).unwrap_or(0) as u64,
            ReadWrite::LoadHalf => self.memory.load_half(address).unwrap_or(0) as i64 as u64,
            ReadWrite::LoadHalfUnsigned => self.memory.load_half(address).unwrap_or(0) as u64,
            ReadWrite::LoadWord => self.memory.load_word(address).unwrap_or(0) as i64 as u64,
            ReadWrite::LoadWordUnsigned => self.memory.load_word(address).unwrap_or(0) as u64,
            ReadWrite::LoadDouble => self.memory.load_double_word(address).unwrap_or(0),
            _ => 0,
        };
    }

    /// Write to memory based on the address provided by the ALU in
    /// [`RiscDatapathState::alu_result`]. The source of the data being written to
    /// memory is determined by [`MemWriteSrc`].
    fn memory_write(&mut self) {
        let address = self.state.alu_result;

        self.state.write_data = self.state.read_data_2;

        // Choose the correct store function based on the RegWidth
        // control signal.
        match self.signals.read_write {
            ReadWrite::StoreByte => {
                self.memory
                    .store_byte(address, self.state.write_data as u8)
                    .ok();
            }
            ReadWrite::StoreHalf => {
                self.memory
                    .store_half(address, self.state.write_data as u16)
                    .ok();
            }
            ReadWrite::StoreWord => {
                self.memory
                    .store_word(address, self.state.write_data as u32)
                    .ok();
            }
            ReadWrite::StoreDouble => {
                self.memory
                    .store_double_word(address, self.state.write_data)
                    .ok();
            }
            _ => (),
        };
    }

    fn calc_general_branch_signal(&mut self) {
        // Assume there is no branch initially.
        self.datapath_signals.general_branch = GeneralBranch::NoBranch;

        if let CpuBranch::YesBranch = self.datapath_signals.cpu_branch {
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
        self.state.new_pc = match self.signals.branch_jump {
            BranchJump::J => self.state.jump_address,
            _ => self.state.mem_mux1_to_mem_mux2,
        };
    }

    // ====================== Writeback (WB) ======================
    /// Write to a register. This will only write if the RegWrite
    /// control signal is set.
    fn register_write(&mut self) {
        // Determine what data will be sent to the register: either
        // the result from the ALU, or data retrieved from memory.
        self.state.data_result = match self.signals.wb_sel {
            WBSel::UseAlu => self.state.alu_result,
            WBSel::UseMemory => self.state.memory_data,
            WBSel::UsePcPlusFour => self.state.pc_plus_4,
            WBSel::UseImmediate => self.state.imm as i64 as u64,
        };

        // Decide to retrieve data either from the main processor or the coprocessor.
        self.state.register_write_data = self.state.data_result;

        // Abort if the RegWrite signal is not set.
        if self.signals.reg_write_en == RegWriteEn::NoWrite {
            return;
        }

        // Determine the destination for the data to write. This is
        // determined by the RegDst control signal.
        self.state.write_register_destination = match self.signals.reg_dst {
            RegDst::Reg1 => self.state.rs1 as usize,
            RegDst::Reg2 => self.state.rs2 as usize,
            RegDst::Reg3 => self.state.rd as usize,
            RegDst::ReturnRegister => 31_usize,
        };

        // If we are attempting to write to register $zero, stop.
        if self.state.write_register_destination == 0 {
            return;
        }

        // Write.
        self.registers.gpr[self.state.write_register_destination] = self.state.register_write_data;
    }

    /// Update the program counter register.
    ///
    /// This function is called from the WB stage.
    fn set_pc(&mut self) {
        self.registers.pc = self.state.new_pc;
    }
}
