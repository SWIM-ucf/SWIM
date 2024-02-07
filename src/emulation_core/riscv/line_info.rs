//! Module for mapping lines in the visual datapath to information
//! and variables in the coded datapath.

use super::super::datapath::VisualDatapath;
use super::datapath::RiscDatapath;

/// A collection of data surrounding a line in the visual datapath.
pub struct LineInformation {
    pub title: String,
    pub description: String,

    /// The value stored in a line. This may not be a 64-bit value, but should
    /// refer to the `bits` field to determine how many bits on the line are
    /// relevant to be displayed.
    pub value: u64,

    /// The number of bits on a given line.
    pub bits: u64,
}

impl VisualDatapath for RiscDatapath {
    type LineInformation = LineInformation;

    fn visual_line_to_data(&self, variable: &str) -> LineInformation {
        match variable {
            "alu_input2" => LineInformation {
                title: String::from("ALU Input 2"),
                description: String::from("The second input to the ALU. This is determined by the ALUSrc control signal to select between register data, a sign-extended and left-shifted immediate value, or a zero-extended immediate value."),
                value: self.state.alu_input2,
                bits: 64,
            },
            "alu_result" => LineInformation {
                title: String::from("ALU Result"),
                description: String::from("The result of the calculation performed by the ALU. This is used either as an address to access memory or as a value that is saved into a register."),
                value: self.state.alu_result,
                bits: 64,
            },
            "data_result" => LineInformation {
                title: String::from("Writeback Data"),
                description: String::from("After finishing processing the instruction, this will either be the ALU result, data from memory, or PC + 4, based on the MemToReg control signal. This data is saved into registers."),
                value: self.state.data_result,
                bits: 64,
            },
            "fpu_alu_result" => LineInformation {
                title: String::from("Floating-Point ALU Result"),
                description: String::from("The result of the calculation performed by the floating-point ALU. This is used as an option to be written to a floating-point register, based on the DataWrite and FpuMemToReg control signals."),
                value: self.coprocessor.state.alu_result,
                bits: 64,
            },
            "fpu_branch_decision" => LineInformation {
                title: String::from("FPU Branch Decision"),
                description: String::from("Based on the true/false branch flag, determines whether to branch. (The FpuBranch control signal must also be set.)"),
                value: self.coprocessor.state.condition_code_mux as u64,
                bits: 1,
            },
            "fpu_branch_flag" => LineInformation {
                title: String::from("Instruction [16] (True/False Branch Flag)"),
                description: String::from("The true/false branch flag of branching coprocessor instructions. This flag specifies whether a floating-point branch instruction is BC1T or BC1F."),
                value: self.coprocessor.state.branch_flag as u64,
                bits: 1,
            },
            "fpu_comparator_result" => LineInformation {
                title: String::from("Floating-Point Comparator Result"),
                description: String::from("The result of the comparison of two floating-point values. This is routed to the \"Condition Code\" (cc) register, and will be written there if the CcWrite control signal is set."),
                value: self.coprocessor.state.comparator_result,
                bits: 64,
            },
            "fpu_condition_code" => LineInformation {
                title: String::from("Condition Code Value"),
                description: String::from("Data retrieved from the \"Condition Code\" (cc) register. This specifies whether a previous conditional instruction was true or false."),
                value: self.coprocessor.state.condition_code_bit as u64,
                bits: 1,
            },
            "fpu_condition_code_inverted" => LineInformation {
                title: String::from("Condition Code Value (Inverted)"),
                description: String::from("Inverted form of the condition code register value."),
                value: self.coprocessor.state.condition_code_bit_inverted as u64,
                bits: 1,
            },
            "fpu_data" => LineInformation {
                title: String::from("Floating-Point Data Register Value"),
                description: String::from("Data retrieved from the \"Data\" register. This register acts as a means to communicate data between the main processor and floating-point coprocessor in MTC1 and MFC1 instructions."),
                value: self.coprocessor.state.fmt as u64,
                bits: 64,
            },
            "fpu_data_writeback" => LineInformation {
                title: String::from("Floating-Point Data Writeback"),
                description: String::from("The value from the floating-point unit's \"Data\" register. Depending on the FpuRegWidth control signal, this will be 64-bit data or sign-extended 32-bit data."),
                value: self.coprocessor.state.data_writeback,
                bits: 64,
            },
            "fpu_destination" => LineInformation {
                title: String::from("Floating-Point Write Register"),
                description: String::from("The register that will be written to, assuming FpuRegWrite is set. Depending on the FpuRegDst control signal, this will consist of the fs, ft, or fd register."),
                value: self.coprocessor.state.destination as u64,
                bits: 5,
            },
            "fpu_fd" => LineInformation {
                title: String::from("Instruction [10-6] (fd)"),
                description: String::from("The fd field. Depending on the FpuRegDst control signal, this will be the register written to in a floating-point operation. This register is used as the destination for most floating-point arithmetic instructions."),
                value: self.coprocessor.state.fd as u64,
                bits: 5,
            },
            "fpu_fmt" => LineInformation {
                title: String::from("Instruction [25-21] (fmt)"),
                description: String::from("The fmt field. This is used to distinguish between single-precision and double-precision floating-point instructions."),
                value: self.coprocessor.state.fmt as u64,
                bits: 5,
            },
            "fpu_fp_register_data_from_main_processor" => LineInformation {
                title: String::from("Writeback Data (To Floating-Point Coprocessor)"),
                description: String::from("This data is written to a floating-point register, given FpuMemToReg is set. This line allows data to load from memory to a floating-point register, specifically in the case of the LWC1 instruction."),
                value: self.coprocessor.state.fp_register_data_from_main_processor,
                bits: 64,
            },
            "fpu_fp_register_to_memory" => LineInformation {
                title: String::from("Memory Write Data (from FPU)"),
                description: String::from("If the MemWriteSrc control signal is set, this data will be written to memory. This is used for the SWC1 instruction."),
                value: self.coprocessor.state.fp_register_to_memory,
                bits: 64,
            },
            "fpu_fs" => LineInformation {
                title: String::from("Instruction [15-11] (fs)"),
                description: String::from("The fs field. Contains the first register to be read for a floating-point instruction."),
                value: self.coprocessor.state.fs as u64,
                bits: 5,
            },
            "fpu_ft" => LineInformation {
                title: String::from("Instruction [20-16] (ft)"),
                description: String::from("The ft field. Contains the second register to be read for a floating-point instruction."),
                value: self.coprocessor.state.ft as u64,
                bits: 5,
            },
            "fpu_new_data" => LineInformation {
                title: String::from("New Floating-Point Data Register Value"),
                description: String::from("Data sent to the \"Data\" register. Depending on the DataSrc control signal, this will either be data from the main processor or the floating-point coprocessor. This register acts as a means to communicate data between the main processor and floating-point coprocessor in MTC1 and MFC1 instructions."),
                value: self.coprocessor.state.fmt as u64,
                bits: 64,
            },
            "fpu_read_data_1" => LineInformation {
                title: String::from("FPU Read Data 1"),
                description: String::from("Data retrieved from the register specified by the fs instruction field. This is used as the first inputs to the floating-point ALU and comparator. This can additionally be written to the \"Data\" register, based on the DataSrc and DataWrite control signals."),
                value: self.coprocessor.state.read_data_1,
                bits: 64,
            },
            "fpu_read_data_2" => LineInformation {
                title: String::from("FPU Read Data 2"),
                description: String::from("Data retrieved from the register specified by the ft instruction field. This is used as the second inputs to the floating-point ALU and comparator. This can additionally be used as data to be written to memory, based on the MemWriteSrc control signal."),
                value: self.coprocessor.state.read_data_2,
                bits: 64,
            },
            "fpu_register_write_data" => LineInformation {
                title: String::from("FPU Register Write Data"),
                description: String::from("Data that will be written to a floating-point register, given that FpuRegWrite is set."),
                value: self.coprocessor.state.register_write_data,
                bits: 64,
            },
            "fpu_register_write_mux_to_mux" => LineInformation {
                title: String::from("FPU Register Write Data (When FpuMemToReg is Unset)"),
                description: String::from("Based on the DataWrite control signal, this will either be the result of the floating-point ALU or the contents of the \"Data\" register. (The \"Data\" register is used for transferring data between the processor and floating-point coprocessor.)"),
                value: self.coprocessor.state.register_write_mux_to_mux,
                bits: 64,
            },
            "fpu_sign_extend_data" => LineInformation {
                title: String::from("Floating-Point Data Register Value (Sign-Extended)"),
                description: String::from("In the case where FpuRegWidth indicates a 32-bit width, this is the bottom 32 bits of the value from the \"Data\" register, then sign-extended to 64 bits."),
                value: self.coprocessor.state.sign_extend_data,
                bits: 64,
            },
            "funct" => LineInformation {
                title: String::from("Instruction [5-0] (funct)"),
                description: String::from("The funct field. Contains the type of operation to execute for R-type instructions."),
                value: self.state.funct as u64,
                bits: 6,
            },
            "imm" => LineInformation {
                title: String::from("Instruction [15-0] (immediate)"),
                description: String::from("The immediate field. Contains the 16-bit constant value used for I-type instructions."),
                value: self.state.imm as u64,
                bits: 16,
            },
            "instruction" => LineInformation {
                title: String::from("Instruction"),
                description: String::from("The currently-loaded instruction. This is broken down into different fields, where each field serves a different purpose in identifying what the instruction does."),
                value: self.state.instruction as u64,
                bits: 32,
            },
            "jump_address" => LineInformation {
                title: String::from("Jump Address"),
                description: String::from("The concatenation of the upper 36 bits of PC + 4 with the lower 26 bits of the instruction, shifted left by 2. This is used as the new PC value for J-type instructions."),
                value: self.state.jump_address,
                bits: 64,
            },
            "lower_26" => LineInformation {
                title: String::from("Instruction [25-0]"),
                description: String::from("The lower 26 bits of instruction. This is used as part of the new PC value for J-type instructions."),
                value: self.state.lower_26 as u64,
                bits: 26,
            },
            "lower_26_shifted_left_by_2" => LineInformation {
                title: String::from("Instruction [25-0] << 2"),
                description: String::from("The lower 26 bits of instruction, shifted left by 2. This is used as part of the new PC value for J-type instructions."),
                value: self.state.lower_26_shifted_left_by_2 as u64,
                bits: 28,
            },
            "mem_mux1_to_mem_mux2" => LineInformation {
                title: String::from("Relative PC Address"),
                description: String::from("Based on the control signals for branching and jumping, this address may be the next PC value. This is used for general non-branching instructions or branch-type instructions."),
                value: self.state.mem_mux1_to_mem_mux2,
                bits: 64,
            },
            "memory_data" => LineInformation {
                title: String::from("Memory Data"),
                description: String::from("The data retrieved from memory, given that the MemRead control signal is set. This may be 32 bits or 64 bits, depending on the RegWidth control signal."),
                value: self.state.memory_data,
                bits: 64,
            },
            "new_pc" => LineInformation {
                title: String::from("New Program Counter"),
                description: String::from("The address of the next instruction to execute. In other words, the next value of the program counter (PC) register."),
                value: self.state.new_pc,
                bits: 64,
            },
            "pc" => LineInformation {
                title: String::from("Program Counter"),
                description: String::from("The address of the currently-executing instruction."),
                value: self.registers.pc,
                bits: 64,
            },
            "pc_plus_4" => LineInformation {
                title: String::from("PC + 4"),
                description: String::from("The address of the currently-executing instruction, plus 4. By default, this will become the next value of the PC register. However, a different address may be used in the case of a branch or jump instruction."),
                value: self.state.pc_plus_4,
                bits: 64,
            },
            "pc_plus_4_upper" => LineInformation {
                title: String::from("PC + 4 [63-28]"),
                description: String::from("The upper 36 bits of PC + 4. This is to be concatenated with the lower 26 bits of the instruction to calculate a jump address."),
                value: self.state.pc_plus_4 & 0xffff_ffff_f000_0000 >> 28,
                bits: 36,
            },
            "ra_id" => LineInformation {
                title: String::from("Return Address Register Index"),
                description: String::from("The value 31. This represents the thirty-second register, the return address register ($ra)."),
                value: 31,
                bits: 5,
            },
            "rd" => LineInformation {
                title: String::from("Instruction [15-11] (rd)"),
                description: String::from("The rd field. Depending on the RegDst control signal, this will be the register written to for an instruction. This register is used as the destination for most R-type instructions."),
                value: self.state.rd as u64,
                bits: 5,
            },
            "read_data_1" => LineInformation {
                title: String::from("Read Data 1"),
                description: String::from("Data retrieved from the register specified by the rs instruction field. Based on the instruction, this may be used as the first input to the ALU, or the next value of the PC register."),
                value: self.state.read_data_1,
                bits: 64,
            },
            "read_data_2" => LineInformation {
                title: String::from("Read Data 2"),
                description: String::from("Data retrieved from the register specified by the rt instruction field. Based on the instruction, this may be used as the second input to the ALU, data written to memory, or data transferred to the floating-point coprocessor."),
                value: self.state.read_data_2,
                bits: 64,
            },
            "register_write_data" => LineInformation {
                title: String::from("Register Write Data"),
                description: String::from("Data that will be written to a general-purpose register, given that RegWrite is set."),
                value: self.state.register_write_data,
                bits: 64,
            },
            "relative_pc_branch" => LineInformation {
                title: String::from("Relative PC Branch Address"),
                description: String::from("The relative address used in branch instructions. This is the sum of PC + 4 and the sign-extended immediate value, shifted left by 2."),
                value: self.state.relative_pc_branch,
                bits: 64,
            },
            "rs" => LineInformation {
                title: String::from("Instruction [25-21] (rs)"),
                description: String::from("The rs field. Contains the first register to be read for an instruction."),
                value: self.state.rs as u64,
                bits: 5,
            },
            "rt" => LineInformation {
                title: String::from("Instruction [20-16] (rt)"),
                description: String::from("The rt field. Contains the second register to be read for an instruction."),
                value: self.state.rt as u64,
                bits: 5,
            },
            "shamt" => LineInformation {
                title: String::from("Instruction [10-6] (shamt)"),
                description: String::from("The shamt (\"shift amount\") field. Specifies the number of bits to shift for those instructions that perform bit-shifting."),
                value: self.state.shamt as u64,
                bits: 5,
            },
            "sign_extend" => LineInformation {
                title: String::from("Sign-Extended Immediate"),
                description: String::from("The immediate field, sign-extended to a 64-bit value."),
                value: self.state.sign_extend,
                bits: 64,
            },
            "sign_extend_shift_left_by_2" => LineInformation {
                title: String::from("Sign-Extended Immediate << 2"),
                description: String::from("The immediate field, sign-extended to a 64-bit value, then shifted left by 2."),
                value: self.state.sign_extend_shift_left_by_2,
                bits: 64,
            },
            "write_data" => LineInformation {
                title: String::from("Memory Write Data"),
                description: String::from("Given that the MemWrite control signal is set, this data will be written to memory."),
                value: self.state.write_data,
                bits: 64,
            },
            "write_register" => LineInformation {
                title: String::from("Write Register"),
                description: String::from("The register that will be written to, assuming RegWrite is set. Depending on the RegDst control signal, this will consist of the rs, rt, or rd register, or 31 (indicating the $ra register)."),
                value: self.state.write_register_destination as u64,
                bits: 5,
            },
            "zero_extended_immediate" => LineInformation {
                title: String::from("Zero-Extended Immediate"),
                description: String::from("The immediate field, zero-extended to a 64-bit value."),
                value: self.state.imm as u64,
                bits: 64,
            },
            _ => LineInformation {
                title: String::from("[Title]"),
                description: String::from("[Description]"),
                value: 0,
                bits: 0,
            },
        }
    }
}
