use super::control_signals::RegWidth;

pub const FUNCT_SYSCALL: u8 = 0b001100;

pub const FUNCT_SLL: u8 = 0b000000;
pub const FUNCT_ADD: u8 = 0b100000;
pub const FUNCT_ADDU: u8 = 0b100001;
pub const FUNCT_SUB: u8 = 0b100010;
pub const FUNCT_AND: u8 = 0b100100;
pub const FUNCT_OR: u8 = 0b100101;
pub const FUNCT_SLT: u8 = 0b101010;
pub const FUNCT_SLTU: u8 = 0b101011;

pub const FUNCT_DADD: u8 = 0b101100;
pub const FUNCT_DADDU: u8 = 0b101101;
pub const FUNCT_DSUB: u8 = 0b101110;
pub const FUNCT_DSUBU: u8 = 0b101111;

pub const FUNCT_JALR: u8 = 0b001001;
pub const FUNCT_JR: u8 = FUNCT_JALR;

/// Used for `MUL` and `MUH`.
pub const FUNCT_SOP30: u8 = 0b011000;

/// Used for `MULU` and `MUHU`.
pub const FUNCT_SOP31: u8 = 0b011001;

/// Used for `DIV` and `MOD`.
pub const FUNCT_SOP32: u8 = 0b011010;

/// Used for `DIVU` and `MODU`.
pub const FUNCT_SOP33: u8 = 0b011011;

/// Used for `DMUL` and `DMUH`.
pub const FUNCT_SOP34: u8 = 0b011100;

/// Used for `DMULU` and `DMUHU`.
pub const FUNCT_SOP35: u8 = 0b011101;

/// Used for `DDIV` and `DMOD`.
pub const FUNCT_SOP36: u8 = 0b011110;

/// Used for `DDIVU` and `DMODU`.
pub const FUNCT_SOP37: u8 = 0b011111;

/// Used for many R-type instructions, like `ADD`, `SUB`, `MUL`, and `DIV`.
pub const OPCODE_SPECIAL: u8 = 0b000000;
/// Used for register-immediate instructions, like `DAHI` and `DATI`.
pub const OPCODE_REGIMM: u8 = 0b000001;

pub const OPCODE_ORI: u8 = 0b001101;
pub const OPCODE_ANDI: u8 = 0b001100;

pub const OPCODE_ADDI: u8 = 0b001000;
pub const OPCODE_ADDIU: u8 = 0b001001;
pub const OPCODE_DADDI: u8 = 0b011000;
pub const OPCODE_DADDIU: u8 = 0b011001;

pub const OPCODE_COP1: u8 = 0b010001;
pub const OPCODE_LUI: u8 = 0b001111;
pub const OPCODE_AUI: u8 = OPCODE_LUI;

// Loading and Storing
pub const OPCODE_LW: u8 = 0b100011;
pub const OPCODE_SW: u8 = 0b101011;
pub const OPCODE_LWC1: u8 = 0b110001;
pub const OPCODE_SWC1: u8 = 0b111001;

// Jump opcodes:
pub const OPCODE_J: u8 = 0b000010;
pub const OPCODE_JAL: u8 = 0b000011;

// Branch OPCODE's
pub const OPCODE_BEQ: u8 = 0b000100;
pub const OPCODE_BNE: u8 = 0b000101;

// "ENC" is short for encoding. There is no formal name for this field
// in the MIPS64 specification, other than the "shamt"/"sa" field that it
// replaces, so this was chosen as the mnemonic for this project.
pub const ENC_MUL: u8 = 0b00010;
pub const ENC_MULU: u8 = 0b00010;
pub const ENC_DIV: u8 = 0b00010;
pub const ENC_DIVU: u8 = 0b00010;
pub const ENC_DMUL: u8 = 0b00010;
pub const ENC_DMULU: u8 = 0b00010;
pub const ENC_DDIV: u8 = 0b00010;
pub const ENC_DDIVU: u8 = 0b00010;

// "RMSUB" is short for register immediate subcode. There is no formal name
// for this field in the MIPS64 specification, other than the "rt" field that
// it replaces, so this was chosen as a mnemonic for this project.
pub const RMSUB_DAHI: u8 = 0b00110;
pub const RMSUB_DATI: u8 = 0b11110;

pub const FUNCTION_ADD: u8 = 0b000000;
pub const FUNCTION_SUB: u8 = 0b000001;
pub const FUNCTION_MUL: u8 = 0b000010;
pub const FUNCTION_DIV: u8 = 0b000011;

// All floating-point c.cond.fmt instructions begin the
// function field with 11.
pub const FUNCTION_C_EQ: u8 = 0b110010;
pub const FUNCTION_C_LT: u8 = 0b111100;
pub const FUNCTION_C_NGE: u8 = 0b111101;
pub const FUNCTION_C_LE: u8 = 0b111110;
pub const FUNCTION_C_NGT: u8 = 0b111111;

// "SUB" is short for operation subcode. Bits 25..21 of some instructions.
/// Floating-point branch conditional.
pub const SUB_BC: u8 = 0b01000;
/// Move word from floating point.
pub const SUB_MF: u8 = 0b00000;
/// Move word to floating point.
pub const SUB_MT: u8 = 0b00100;
/// Doubleword move from floating point.
pub const SUB_DMF: u8 = 0b00001;
/// Doubleword move to floating point.
pub const SUB_DMT: u8 = 0b00101;

pub const FMT_SINGLE: u8 = 16;
pub const FMT_DOUBLE: u8 = 17;

/// Return the register width associated to an instruction
/// with the given `funct` code.
///
/// Returns [`None`] if the `funct` code is not supported.
pub fn reg_width_by_funct(funct: u8) -> Option<RegWidth> {
    match funct {
        // `syscall` does not have a register width associated with it,
        // but is set for the purposes of a default signal value.
        FUNCT_SYSCALL => Some(RegWidth::DoubleWord),
        FUNCT_ADD | FUNCT_ADDU | FUNCT_SUB | FUNCT_SLL => Some(RegWidth::Word),
        FUNCT_AND | FUNCT_OR | FUNCT_SLT | FUNCT_SLTU => Some(RegWidth::DoubleWord),
        FUNCT_DADD | FUNCT_DSUB => Some(RegWidth::DoubleWord),
        FUNCT_DADDU | FUNCT_DSUBU => Some(RegWidth::DoubleWord),
        FUNCT_SOP30 | FUNCT_SOP31 | FUNCT_SOP32 | FUNCT_SOP33 => Some(RegWidth::Word),
        FUNCT_SOP34 | FUNCT_SOP35 | FUNCT_SOP36 | FUNCT_SOP37 => Some(RegWidth::DoubleWord),
        _ => None,
    }
}
