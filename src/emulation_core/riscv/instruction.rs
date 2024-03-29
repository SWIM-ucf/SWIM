//! Abstract representation of an instruction.

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

/// - funct7:
/// - rs2: CPU register - used as a source to read from in the register file.
/// - rs1: CPU register - used as a source to read from in the register file.
/// - funct3:
/// - rd: CPU register - can be used as a destination for the result of executed instructions.
/// - opcode: Determines the type of instruction executed. This is typically 0110011 in R-type instructions.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RType {
    pub funct7: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IType {
    pub imm: u16,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SType {
    pub imm1: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm2: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BType {
    pub imm: u16,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UType {
    pub imm: i32,
    pub rd: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JType {
    pub imm: i32,
    pub rd: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct R4Type {
    pub rs3: u8,
    pub funct2: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
    pub op: u8,
}

#[derive(Clone, Debug, PartialEq)]
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
            OPCODE_OP | OPCODE_OP_32 => Ok(Instruction::RType(RType {
                funct7: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // I-type instructions:
            OPCODE_IMM | OPCODE_IMM_32 | OPCODE_JALR | OPCODE_LOAD | OPCODE_SYSTEM => {
                Ok(Instruction::IType(IType {
                    imm: (value >> 20) as u16,
                    rs1: ((value >> 15) & 0x1f) as u8,
                    funct3: ((value >> 12) & 0x07) as u8,
                    rd: ((value >> 7) & 0x1f) as u8,
                    op: (value & 0x7f) as u8,
                }))
            }

            // S-type instruction:
            OPCODE_STORE => Ok(Instruction::SType(SType {
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

            _ => Err(format!("opcode `{op}` not supported")),
        }
    }
}
