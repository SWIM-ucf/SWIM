use super::super::datapath::Datapath;
use super::{control_signals::*, memory::Memory, registers::Registers};

/// An implementation of a datapath for the MIPS64 ISA.
///
/// It is assumed that while moving through stages, only one
/// instruction will be active any any given point in time. Due to this,
/// we consider the datapath to be a "pseudo-single-cycle datapath."
#[derive(Default)]
pub struct MipsDatapath {
    pub registers: Registers,
    pub memory: Memory,
    pub instruction: u32,
    pub signals: ControlSignals,

    opcode: u32,
    rs: u32,
    rt: u32,
    rd: u32,
    shamt: u32,
    funct: u32,
    imm: u32,

    read_data_1: u64,
    read_data_2: u64,
    sign_extend: u64,

    alu_result: u64,
    memory_data: u64,
    data_result: u64,

    current_stage: Stage,
}

/// The possible stages the datapath could be in during execution.
#[derive(Default, Copy, Clone, Eq, PartialEq)]
enum Stage {
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
fn error(message: &str) {
    panic!("{}", message);
}

impl Datapath for MipsDatapath {
    type RegisterData = u64;
    type RegisterEnum = super::registers::RegisterType;
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
}

impl MipsDatapath {
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
    }

    /// Stage 3 of 5: Execute (EX)
    ///
    /// Execute the current instruction with some arithmetic operation.
    fn stage_execute(&mut self) {
        self.alu();
    }

    /// Stage 4 of 5: Memory (MEM)
    ///
    /// Read or write to memory.
    fn stage_memory(&mut self) {
        // TODO
        /*
        if self.signals.mem_read == 1{
            self.memory_read();
        }

        if self.signals.mem_write == 1 {
            self.memory_write();
        }
        */
    }

    /// Stage 5 of 5: Writeback (WB)
    ///
    /// Write the result of the instruction's operation to a register,
    /// if desired. Additionally, set the PC for the next instruction.
    fn stage_writeback(&mut self) {
        self.register_write();
        self.set_pc();
    }

    /// Load the raw binary instruction from memory and into the
    /// datapath. If there is an error with loading the word, assume
    /// the instruction to be bitwise zero and error.
    fn instruction_fetch(&mut self) {
        self.instruction = match self.memory.load_word(self.registers.pc) {
            Ok(data) => data,
            Err(e) => {
                error(e.as_str());
                0
            }
        }
    }

    /// Decode an instruction into its individual fields.
    fn instruction_decode(&mut self) {
        // TODO: Use an enum and structs rather than individual fields.
        self.opcode = (self.instruction >> 26) & 0b111111;
        self.rs = (self.instruction >> 21) & 0b11111;
        self.rt = (self.instruction >> 16) & 0b11111;
        self.rd = (self.instruction >> 11) & 0b11111;
        self.shamt = (self.instruction >> 6) & 0b11111;
        self.funct = self.instruction & 0b111111;
        self.imm = self.instruction & 0xFFFF;
    }

    /// Extend the sign of a 16-bit value to the other 48 bits of a
    /// 64-bit value.
    fn sign_extend(&mut self) {
        // Is the value negative or positive? Check sign bit

        // 0000 0000 0000 0000 1000 0000 0000 0000
        // 0x00008000

        self.sign_extend = if (self.imm & 0x00008000) >> 15 == 0 {
            self.imm as u64
        } else {
            (self.imm as u64) | 0xFFFF_FFFF_FFFF_0000
        }
    }

    /// Set the control signals for the datapath based on the
    /// instruction's opcode.
    fn set_control_signals(&mut self) {
        match self.opcode {
            // R-type instructions (add, sub, mul, div, and, or, slt, sltu)
            0 => {
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
                self.signals.reg_width = RegWidth::Word;
                self.signals.reg_write = RegWrite::YesWrite;
            }
            _ => error("Instruction not supported."),
        }
    }

    /// Read the registers as specified from the instruction and pass
    /// the data into the datapath.
    fn read_registers(&mut self) {
        let reg1 = self.rs as usize;
        let reg2 = self.rt as usize;

        self.read_data_1 = self.registers.gpr[reg1];
        self.read_data_2 = self.registers.gpr[reg2];

        // Truncate the variable data if a 32-bit word is requested.
        if let RegWidth::Word = self.signals.reg_width {
            self.read_data_1 = self.registers.gpr[reg1] as u32 as u64;
            self.read_data_2 = self.registers.gpr[reg2] as u32 as u64;
        }
    }

    /// Set the ALU control signal based on the ALU operation signal.
    fn set_alu_control(&mut self) {
        self.signals.alu_control = match self.signals.alu_op {
            AluOp::Addition => AluControl::Addition,
            AluOp::Subtraction => AluControl::Subtraction,
            AluOp::SetOnLessThanSigned => AluControl::SetOnLessThanSigned,
            AluOp::SetOnLessThanUnsigned => AluControl::SetOnLessThanUnsigned,
            AluOp::And => AluControl::And,
            AluOp::Or => AluControl::Or,
            AluOp::LeftShift16 => AluControl::LeftShift16,
            AluOp::UseFunctField => match self.funct {
                0b100000 => AluControl::Addition,
                0b100010 => AluControl::Subtraction,
                0b100100 => AluControl::And,
                0b100101 => AluControl::Or,
                0b101010 => AluControl::SetOnLessThanSigned,
                0b101011 => AluControl::SetOnLessThanUnsigned,
                0b011010 | 0b011110 => match self.shamt {
                    0b00010 => AluControl::DivisionSigned,
                    _ => {
                        error("Unsupported funct");
                        AluControl::Addition // Stub
                    }
                },
                0b011011 | 0b011111 => match self.shamt {
                    0b00010 => AluControl::DivisionUnsigned,
                    _ => {
                        error("Unsupported funct");
                        AluControl::Addition // Stub
                    }
                },
                0b011000 | 0b011100 => match self.shamt {
                    0b00010 => AluControl::MultiplicationSigned,
                    _ => {
                        error("Unsupported funct");
                        AluControl::Addition // Stub
                    }
                },
                0b011001 | 0b011101 => match self.shamt {
                    0b00010 => AluControl::MultiplicationUnsigned,
                    _ => {
                        error("Unsupported funct");
                        AluControl::Addition // Stub
                    }
                },
                _ => {
                    error("Unsupported funct");
                    AluControl::Addition // Stub
                }
            },
        };
    }

    /// Perform an ALU operation.
    fn alu(&mut self) {
        // TODO: Support alternating between 32-bit and 64-bit operations.

        // Left shift the immediate value based on the ImmShift control signal.
        let alu_immediate = match self.signals.imm_shift {
            ImmShift::Shift0 => self.sign_extend,
            ImmShift::Shift16 => self.sign_extend << 16,
            ImmShift::Shift32 => self.sign_extend << 32,
            ImmShift::Shift48 => self.sign_extend << 48,
        };

        // Specify the inputs for the operation. The first will always
        // be the first register, but the second may be either the
        // second register or the sign-extended immediate value.
        let mut input1 = self.read_data_1;
        let mut input2 = match self.signals.alu_src {
            AluSrc::ReadRegister2 => self.read_data_2,
            AluSrc::ExtendedImmediate => alu_immediate,
        };

        // Truncate the inputs if 32-bit operations are expected.
        if let RegWidth::Word = self.signals.reg_width {
            input1 = input1 as i32 as u64;
            input2 = input2 as i32 as u64;
        }

        // Set the result.
        self.alu_result = match self.signals.alu_control {
            AluControl::Addition => input1.wrapping_add(input2) as u64,
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
            self.alu_result = self.alu_result as i32 as i64 as u64;
        }

        // TODO: Set the zero bit.
    }

    /// Write to a register. This will only write if the RegWrite
    /// control signal is set.
    fn register_write(&mut self) {
        // Determine what data will be sent to the register: either
        // the result from the ALU, or data retrieved from memory.
        self.data_result = match self.signals.mem_to_reg {
            MemToReg::UseAlu => self.alu_result,
            MemToReg::UseMemory => self.memory_data,
        };

        // Abort if the RegWrite signal is not set.
        if self.signals.reg_write == RegWrite::NoWrite {
            return;
        }

        // Determine the destination for the data to write. This is
        // determined by the RegDst control signal.
        let destination = match self.signals.reg_dst {
            RegDst::Reg1 => self.rs as usize,
            RegDst::Reg2 => self.rt as usize,
            RegDst::Reg3 => self.rd as usize,
        };

        // If we are attempting to write to register $zero, stop.
        if destination == 0 {
            return;
        }

        // If a 32-bit word is requested, ensure data is truncated and sign-extended.
        if let RegWidth::Word = self.signals.reg_width {
            self.data_result = self.data_result as i32 as u64;
        }

        // Write.
        self.registers.gpr[destination] = self.data_result;
    }

    /// Update the program counter register. At the moment, this only
    /// increments the PC by 4 and does not support branching or
    /// jumping.
    fn set_pc(&mut self) {
        self.registers.pc += 4;
    }
}
