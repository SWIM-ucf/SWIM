/// Used for R-type instructions.
pub const OPCODE_OP: u8 = 0b0110011;
pub const OPCODE_OP_32: u8 = 0b0111011;
pub const OPCODE_OP_FP: u8 = 0b1010011;

/// Used for I-type instructions.
pub const OPCODE_IMM: u8 = 0b0010011;
pub const OPCODE_IMM_32: u8 = 0b0011011;
// JALR
pub const OPCODE_JALR: u8 = 0b1100111;
// LOAD
pub const OPCODE_LOAD: u8 = 0b0000011;
pub const OPCODE_LOAD_FP: u8 = 0b0000111;
// SYSTEM
pub const OPCODE_SYSTEM: u8 = 0b1110011;

/// Used for S-type instructions.
pub const OPCODE_STORE: u8 = 0b0100011;
pub const OPCODE_STORE_FP: u8 = 0b0100111;

/// Used for B-type instructions.
pub const OPCODE_BRANCH: u8 = 0b1100011;

/// Used for U-type instructions.
// LUI
pub const OPCODE_LUI: u8 = 0b0110111;
// AUIPC
pub const OPCODE_AUIPC: u8 = 0b0010111;

/// Used for J-type instructions.
pub const OPCODE_JAL: u8 = 0b1101111;

/// Used for R4-type instructions.
// FMADD.S
pub const OPCODE_MADD: u8 = 0b1000011;
// FMSUB.S
pub const OPCODE_MSUB: u8 = 0b1000111;
// FNMSUB.S
pub const OPCODE_NMSUB: u8 = 0b1001011;
// FNMADD.S
pub const OPCODE_NMADD: u8 = 0b1001111;

/// Not a Number
pub const RISC_NAN: u32 = 0x7fc00000;
