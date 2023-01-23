use super::control_signals::RegWidth;

pub const FUNCT_ADD: u8 = 0b100000;
pub const FUNCT_SUB: u8 = 0b100010;
pub const FUNCT_AND: u8 = 0b100100;
pub const FUNCT_OR: u8 = 0b100101;
pub const FUNCT_SLT: u8 = 0b101010;
pub const FUNCT_SLTU: u8 = 0b101011;

pub const FUNCT_DADD: u8 = 0b101100;
pub const FUNCT_DSUB: u8 = 0b101110;

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
pub const OPCODE_ORI: u8 = 0b001101;
pub const OPCODE_COP1: u8 = 0b010001;
pub const OPCODE_LW: u8 = 0b100011;
pub const OPCODE_SW: u8 = 0b101011;
pub const OPCODE_LWC1: u8 = 0b110001;
pub const OPCODE_SWC1: u8 = 0b111001;

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

pub const FUNCTION_ADD: u8 = 0b000000;
pub const FUNCTION_SUB: u8 = 0b000001;
pub const FUNCTION_MUL: u8 = 0b000010;
pub const FUNCTION_DIV: u8 = 0b000011;

pub const FMT_SINGLE: u8 = 16;
pub const FMT_DOUBLE: u8 = 17;

/// Return the register width associated to an instruction
/// with the given `funct` code.
///
/// Returns [`None`] if the `funct` code is not supported.
pub fn reg_width_by_funct(funct: u8) -> Option<RegWidth> {
    match funct {
        FUNCT_ADD | FUNCT_SUB => Some(RegWidth::Word),
        FUNCT_AND | FUNCT_OR | FUNCT_SLT | FUNCT_SLTU => Some(RegWidth::DoubleWord),
        FUNCT_DADD | FUNCT_DSUB => Some(RegWidth::DoubleWord),
        FUNCT_SOP30 | FUNCT_SOP31 | FUNCT_SOP32 | FUNCT_SOP33 => Some(RegWidth::Word),
        FUNCT_SOP34 | FUNCT_SOP35 | FUNCT_SOP36 | FUNCT_SOP37 => Some(RegWidth::DoubleWord),
        _ => None,
    }
}
