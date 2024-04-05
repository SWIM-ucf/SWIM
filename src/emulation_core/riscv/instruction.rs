//! Abstract representation of an instruction.

use serde::{Deserialize, Serialize};

use super::constants::*;

/// Register (R-Type) Instruction
///
/// ```text
/// 31           25   24       20   19       15   14     12   11        7   6           0
/// ┌───────────────┬─────────────┬─────────────┬───────────┬─────────────┬───────────────┐
/// │    funct7     │     rs2     │     rs1     │  funct3   │     rd      │    opcode     │
/// │               │             │             │           │             │               │
/// └───────────────┴─────────────┴─────────────┴───────────┴─────────────┴───────────────┘
///         7              5             5           3             5              7
/// ```
///

/// - funct7: 7-bit Function field uses to specify options or operations.
/// - rs2: CPU register - used as a source to read from in the register file.
/// - rs1: CPU register - used as a source to read from in the register file.
/// - funct3: 3-bit Function field uses to specify options or operations.
/// - rd: CPU register - can be used as a destination for the result of executed instructions.
/// - opcode: Determines the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RType {
    pub funct7: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub op: u8,
}

/// Immediate (I-Type) Instruction
///
/// ```text
/// 31                         20   19       15   14     12   11        7   6           0
/// ┌─────────────────────────────┬─────────────┬───────────┬─────────────┬───────────────┐
/// │           imm[11:0]         │     rs1     │  funct3   │     rd      │    opcode     │
/// │                             │             │           │             │               │
/// └─────────────────────────────┴─────────────┴───────────┴─────────────┴───────────────┘
///                 12                    5           3             5              7
/// ```
///

/// - imm[11:0]: 12-bit immediate value, sign-extended by 20 bits.
/// - rs1: CPU register - used as a source to read from in the register file.
/// - funct3: 3-bit Function field uses to specify options or operations.
/// - rd: CPU register - can be used as a destination for the result of executed instructions.
/// - opcode: Determines the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IType {
    pub imm: u16,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub op: u8,
}

/// Store (S-Type) Instruction
///
/// ```text
/// 31           25   24       20   19       15   14     12   11        7   6           0
/// ┌───────────────┬─────────────┬─────────────┬───────────┬─────────────┬───────────────┐
/// │   imm[11:5]   │     rs2     │     rs1     │  funct3   │  imm[4:0]   │    opcode     │
/// │               │             │             │           │             │               │
/// └───────────────┴─────────────┴─────────────┴───────────┴─────────────┴───────────────┘
///         7              5             5           3             5              7
/// ```
///

/// - imm[11:5]: Upper 7-bits of the 12-bit immediate.
/// - rs2: CPU register - used as a source to read from in the register file.
/// - rs1: CPU register - used as a source to read from in the register file.
/// - funct3: 3-bit Function field uses to specify options or operations.
/// - imm[4:0]: Lower 5-bits of the 12-bit immediate
/// - opcode: Determines the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SType {
    pub imm1: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm2: u8,
    pub op: u8,
}

/// Branch (B-Type) Instruction
///
/// ```text
/// 31                         20   19       15   14     12   11        7   6           0
/// ┌─────────────────────────────┬─────────────┬───────────┬─────────────┬───────────────┐
/// │           imm[11:0]         │     rs2     │  funct3   │     rs1     │    opcode     │
/// │                             │             │           │             │               │
/// └─────────────────────────────┴─────────────┴───────────┴─────────────┴───────────────┘
///                 12                    5           3             5              7
/// ```
///

/// - imm[11:0]: 12-bit immediate value, sign-extended by 20 bits.
/// - rs1: CPU register - used as a source to read from in the register file.
/// - funct3: 3-bit Function field uses to specify options or operations.
/// - rs2: CPU register - used as a source to read from in the register file.
/// - opcode: Determines the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BType {
    pub imm: u16,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub op: u8,
}

/// Upper Immediate (U-Type) Instruction
///
/// ```text
/// 31                                                   12   11        7   6           0
/// ┌───────────────────────────────────────────────────────┬─────────────┬───────────────┐
/// │                       imm[31:12]                      │     rd      │    opcode     │
/// │                                                       │             │               │
/// └───────────────────────────────────────────────────────┴─────────────┴───────────────┘
///                              20                                5              7
/// ```
///

/// - imm[31:12]: 20-bit upper immediate.
/// - rd: CPU register - can be used as a destination for the result of executed instructions.
/// - opcode: Determines the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UType {
    pub imm: i32,
    pub rd: u8,
    pub op: u8,
}

/// Jump (J-Type) Instruction
///
/// ```text
/// 31                                                   12   11        7   6           0
/// ┌───────────────────────────────────────────────────────┬─────────────┬───────────────┐
/// │                           imm                         │     rd      │    opcode     │
/// │                                                       │             │               │
/// └───────────────────────────────────────────────────────┴─────────────┴───────────────┘
///                              20                                5              7
/// ```
///

/// - imm: 20-bit immediate.
/// - rd: CPU register - can be used as a destination for the result of executed instructions.
/// - opcode: Determines the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JType {
    pub imm: i32,
    pub rd: u8,
    pub op: u8,
}

/// Fused-Register (R4-Type) Instruction
///
/// ```text
/// 31     27 26  25  24       20   19       15   14     12   11        7   6           0
/// ┌────────┬──────┬─────────────┬─────────────┬───────────┬─────────────┬───────────────┐
/// │  rs3   │funct2│     rs2     │     rs1     │  funct3   │     rd      │    opcode     │
/// │        │      │             │             │           │             │               │
/// └────────┴──────┴─────────────┴─────────────┴───────────┴─────────────┴───────────────┘
///      5       2         5             5           3             5              7
/// ```
///

/// - rs3: CPU register - used as a source to read from in the register file.
/// - funct2: 2-bit Function field uses to specify options or operations.
/// - rs2: CPU register - used as a source to read from in the register file.
/// - rs1: CPU register - used as a source to read from in the register file.
/// - funct3: 3-bit Function field uses to specify options or operations.
/// - rd: CPU register - can be used as a destination for the result of executed instructions.
/// - opcode: Determines the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct R4Type {
    pub rs3: u8,
    pub funct2: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub op: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
    R4Type(R4Type),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::RType(RType::default())
    }
}

impl TryFrom<u32> for Instruction {
    type Error = String;

    /// Based on the opcode, convert a binary instruction into a struct representation.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let op = (value & 0x7f) as u8;
        match op {
            // R-type instructions:
            OPCODE_OP | OPCODE_OP_32 | OPCODE_OP_FP => Ok(Instruction::RType(RType {
                funct7: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // I-type instructions:
            OPCODE_IMM | OPCODE_IMM_32 | OPCODE_JALR | OPCODE_LOAD | OPCODE_SYSTEM
            | OPCODE_LOAD_FP => Ok(Instruction::IType(IType {
                imm: (value >> 20) as u16,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // S-type instruction:
            OPCODE_STORE | OPCODE_STORE_FP => Ok(Instruction::SType(SType {
                imm1: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                imm2: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // B-type instruction:
            OPCODE_BRANCH => Ok(Instruction::BType(BType {
                imm: (value >> 20) as u16,
                rs1: ((value >> 7) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rs2: ((value >> 15) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // U-type instruction:
            OPCODE_LUI | OPCODE_AUIPC => Ok(Instruction::UType(UType {
                imm: (value >> 12) as i32,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // J-type instruction:
            OPCODE_JAL => Ok(Instruction::JType(JType {
                imm: (value >> 12) as i32,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // R4-type instruction:
            OPCODE_MADD | OPCODE_MSUB | OPCODE_NMSUB | OPCODE_NMADD => {
                Ok(Instruction::R4Type(R4Type {
                    rs3: (value >> 27) as u8,
                    funct2: ((value >> 25) & 0x3) as u8,
                    rs2: ((value >> 20) & 0x1f) as u8,
                    rs1: ((value >> 15) & 0x1f) as u8,
                    funct3: ((value >> 12) & 0x07) as u8,
                    rd: ((value >> 7) & 0x1f) as u8,
                    op: (value & 0x7f) as u8,
                }))
            }

            _ => Err(format!("opcode `{op}` not supported")),
        }
    }
}
