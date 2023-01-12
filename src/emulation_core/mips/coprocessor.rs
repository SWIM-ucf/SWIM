//! Implementation of a MIPS64 floating-point coprocessor.

use super::control_signals::floating_point::*;
use super::datapath::error;

/// An implementation of a floating-point coprocessor for the MIPS64 ISA.
///
/// Different from the main processor, much of the functionality of the coprocessor
/// is controlled remotely using its available API calls.
#[derive(Default)]
pub struct MipsFpCoprocessor {
    instruction: u32,
    signals: FpuControlSignals,

    fpr: [u64; 32],
    condition_code: u64,
    data: u64,

    op: u32,
    fmt: u32,
    fs: u32,
    ft: u32,
    fd: u32,
    function: u32,

    data_from_main_processor: u64,
    main_memory_data: u64,
    read_data_1: u64,
    read_data_2: u64,
    alu_result: u64,
    comparator_result: u64,
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
    pub fn set_instruction(&mut self, instruction: u32) {
        self.instruction = instruction;
    }

    /// Decode an instruction into its individual fields.
    fn instruction_decode(&mut self) {
        self.op = (self.instruction >> 26) & 0b111111;
        self.fmt = (self.instruction >> 21) & 0b11111;
        self.fs = (self.instruction >> 11) & 0b11111;
        self.ft = (self.instruction >> 16) & 0b11111;
        self.fd = (self.instruction >> 6) & 0b11111;
        self.function = self.instruction & 0b111111;
    }

    /// Set the [`FpuRegWidth`] control signal based on the `fmt` field in
    /// the instruction.
    fn set_reg_width(&mut self) {
        self.signals.fpu_reg_width = match self.fmt {
            16 => FpuRegWidth::Word,
            17 => FpuRegWidth::DoubleWord,
            _ => {
                error(format!("{} is an invalid fmt value", self.fmt).as_str());
                FpuRegWidth::default()
            }
        }
    }

    /// Sets the data line between the main processor and the `Data` register. This
    /// is then used if deciding data from the main processor should go into the `Data`
    /// register.
    pub fn set_data_from_main_processor(&mut self, data: u64) {
        self.data_from_main_processor = data;
    }

    /// Gets the contents of the data line between the `Data` register and the multiplexer
    /// in the main processor controlled by the [`DataWrite`] control signal.
    pub fn get_main_memory_data(&mut self) -> u64 {
        self.main_memory_data
    }

    /// Defines the control signals to set when the coprocessor is not being used.
    fn set_noop_control_signals(&mut self) {
        self.signals.cc = Cc::Cc0;
        self.signals.cc_write = CcWrite::NoWrite;
        self.signals.data_src = DataSrc::FloatingPointUnit;
        self.signals.data_write = DataWrite::NoWrite;
        self.signals.fpu_alu_op = FpuAluOp::AdditionOrEqual;
        self.signals.fpu_branch = FpuBranch::NoBranch;
        self.signals.fpu_mem_to_reg = FpuMemToReg::UseDataWrite;
        self.signals.fpu_reg_dst = FpuRegDst::Reg2;
        self.signals.fpu_reg_write = FpuRegWrite::NoWrite;
    }

    /// Set the control signals of the processor based on the instruction opcode and function
    /// control signals.
    fn set_control_signals(&mut self) {
        match self.op {
            // COP1 (Coprocessor 1 instruction)
            0b010001 => match self.function {
                // ADD
                0b000000 => {
                    self.signals.cc = Cc::Cc0;
                    self.signals.cc_write = CcWrite::NoWrite;
                    self.signals.data_src = DataSrc::FloatingPointUnit;
                    self.signals.data_write = DataWrite::NoWrite;
                    self.signals.fpu_alu_op = FpuAluOp::AdditionOrEqual;
                    self.signals.fpu_branch = FpuBranch::NoBranch;
                    self.signals.fpu_mem_to_reg = FpuMemToReg::UseDataWrite;
                    self.signals.fpu_reg_dst = FpuRegDst::Reg2;
                    self.signals.fpu_reg_write = FpuRegWrite::YesWrite;
                    self.set_reg_width();
                }
                // Unrecognized format code. Perform no operation.
                _ => self.set_noop_control_signals(),
            },
            // Unrecognized opcode. This may be intended for the main processing unit.
            // Perform no operation.
            _ => self.set_noop_control_signals(),
        }
    }

    /// Read the registers as specified from the instruction and pass
    /// the data into the datapath.
    fn read_registers(&mut self) {
        let reg1 = self.fs as usize;
        let reg2 = self.ft as usize;

        self.read_data_1 = self.fpr[reg1];
        self.read_data_2 = self.fpr[reg2];

        // Truncate the variable data if a 32-bit word is requested.
        if let FpuRegWidth::Word = self.signals.fpu_reg_width {
            self.read_data_1 = self.fpr[reg1] as u32 as u64;
            self.read_data_2 = self.fpr[reg2] as u32 as u64;
        }
    }

    /// Perform an ALU operation.
    fn alu(&mut self) {
        let mut input1 = self.read_data_1;
        let mut input2 = self.read_data_2;

        // Truncate the inputs if 32-bit operations are expected.
        if let FpuRegWidth::Word = self.signals.fpu_reg_width {
            input1 = input1 as u32 as u64;
            input2 = input2 as u32 as u64;
        }

        self.alu_result = match self.signals.fpu_alu_op {
            FpuAluOp::AdditionOrEqual => match self.signals.fpu_reg_width {
                FpuRegWidth::Word => ((input1 as f32) + (input2 as f32)) as u64,
                FpuRegWidth::DoubleWord => ((input1 as f64) + (input2 as f64)) as u64,
            },
            _ => todo!("Unimplemented operation"),
        }
    }

    /// Perform a comparison.
    fn comparator(&mut self) {
        let mut input1 = self.read_data_1;
        let mut input2 = self.read_data_2;

        // Truncate the inputs if 32-bit operations are expected.
        if let FpuRegWidth::Word = self.signals.fpu_reg_width {
            input1 = input1 as u32 as u64;
            input2 = input2 as u32 as u64;
        }

        self.comparator_result = match self.signals.fpu_alu_op {
            FpuAluOp::AdditionOrEqual => (input1 == input2) as u64,
            _ => todo!("Unimplemented operation"),
        }
    }

    /// Write to the `Data` register. This register is used to transfer data between
    /// the main processor and the coprocessor.
    fn write_data(&mut self) {
        if let DataWrite::NoWrite = self.signals.data_write {
            return;
        }

        let data = match self.signals.data_src {
            DataSrc::FloatingPointUnit => self.read_data_1 as u32,
            DataSrc::MainProcessorUnit => self.data_from_main_processor as u32,
        };

        self.condition_code = data as u64;
    }

    /// Set the condition code (CC) register based on the result from the comparator.
    fn write_condition_code(&mut self) {
        self.condition_code = self.comparator_result;
    }

    /// Write data to the floating-point register file.
    fn register_write(&mut self) {
        if let FpuRegWrite::NoWrite = self.signals.fpu_reg_write {
            return;
        }

        let destination = match self.signals.fpu_reg_dst {
            FpuRegDst::Reg1 => self.fs as usize,
            FpuRegDst::Reg2 => self.fd as usize,
        };

        let register_data = match self.signals.fpu_mem_to_reg {
            FpuMemToReg::UseDataWrite => match self.signals.data_write {
                DataWrite::NoWrite => self.alu_result,
                DataWrite::YesWrite => self.data,
            },
            FpuMemToReg::UseMemory => self.main_memory_data,
        };

        self.fpr[destination] = register_data;
    }
}
