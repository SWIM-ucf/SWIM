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

/// Used for R-type instructions.
pub const OPCODE_OP: u8 = 0b0110011;
pub const OPCODE_OP_32: u8 = 0b0111011;

/// Used for I-type instructions.
pub const OPCODE_IMM: u8 = 0b0010011;
pub const OPCODE_IMM_32: u8 = 0b0011011;
// JALR
pub const OPCODE_JALR: u8 = 0b1100111;
// LOAD
pub const OPCODE_LOAD: u8 = 0b0000011;
// SYSTEM
pub const OPCODE_SYSTEM: u8 = 0b1110011;

/// Used for S-type instructions.
pub const OPCODE_STORE: u8 = 0b0100011;

/// Used for B-type instructions.
pub const OPCODE_BRANCH: u8 = 0b1100011;

/// Used for U-type instructions.
// LUI
pub const OPCODE_LUI: u8 = 0b0110111;
// AUIPC
pub const OPCODE_AUIPC: u8 = 0b0010111;

/// Used for J-type instructions.
pub const OPCODE_JAL: u8 = 0b1101111;

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
