//! Abstract representation of an instruction.

use crate::parser::parser_structs_and_enums::RISCV_GP_REGISTERS;

use super::constants::*;

/// Register (R-Type) RiscInstruction
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
    pub imm1: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm2: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UType {
    pub imm: u32,
    pub rd: u8,
    pub op: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JType {
    pub imm: u32,
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
pub enum RiscInstruction {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
    R4Type(R4Type),
}

impl Default for RiscInstruction {
    fn default() -> Self {
        RiscInstruction::RType(RType::default())
    }
}

impl TryFrom<u32> for RiscInstruction {
    type Error = String;

    /// Based on the opcode, convert a binary instruction into a struct representation.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let op = (value & 0x7f) as u8;
        match op {
            // R-type instructions:
            OPCODE_OP | OPCODE_OP_32 => Ok(RiscInstruction::RType(RType {
                funct7: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // I-type instructions:
            OPCODE_IMM | OPCODE_IMM_32 | OPCODE_JALR | OPCODE_LOAD | OPCODE_SYSTEM => {
                Ok(RiscInstruction::IType(IType {
                    imm: (value >> 20) as u16,
                    rs1: ((value >> 15) & 0x1f) as u8,
                    funct3: ((value >> 12) & 0x07) as u8,
                    rd: ((value >> 7) & 0x1f) as u8,
                    op: (value & 0x7f) as u8,
                }))
            }

            // S-type instruction:
            OPCODE_STORE => Ok(RiscInstruction::SType(SType {
                imm1: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                imm2: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // B-type instruction:
            OPCODE_BRANCH => Ok(RiscInstruction::BType(BType {
                imm1: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                imm2: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // U-type instruction:
            OPCODE_LUI | OPCODE_AUIPC => Ok(RiscInstruction::UType(UType {
                imm: value >> 12,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // J-type instruction:
            OPCODE_JAL => Ok(RiscInstruction::JType(JType {
                imm: value >> 12,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            _ => Err(format!("opcode `{op}` not supported")),
        }
    }
}

impl RiscInstruction {
    // Add fence, fence.i
    // Add RV32F
    pub fn get_string_version(value: u32) -> Result<String, String> {
        let mut string_version = String::new();

        let struct_representation = RiscInstruction::try_from(value)?;

        match struct_representation {
            RiscInstruction::RType(r_type) => {
                let rs1 = find_register_name(r_type.rs1).unwrap();
                let rs2 = find_register_name(r_type.rs2).unwrap();
                let rd = find_register_name(r_type.rd).unwrap();
                match r_type.op {
                    OPCODE_OP => match r_type.funct3 {
                        0 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "add", rd, rs1, rs2));
                            }
                            0b0100000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "sub", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "mul", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        1 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "sll", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "mulh", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        2 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "slt", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "mulhsu", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        3 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "sltu", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "mulhu", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        4 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "xor", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "div", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        5 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "srl", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "divu", rd, rs1, rs2));
                            }
                            0b0100000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "sra", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        6 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "or", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "rem", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        7 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "and", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "remu", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    // RISCV64I
                    OPCODE_OP_32 => match r_type.funct3 {
                        0 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "addw", rd, rs1, rs2));
                            }
                            0b0100000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "subw", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "mulw", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        1 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "sllw", rd, rs1, rs2));
                        }
                        4 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "divw", rd, rs1, rs2));
                        }
                        5 => match r_type.funct7 {
                            0b0000000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "srlw", rd, rs1, rs2));
                            }
                            0b0100000 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "sraw", rd, rs1, rs2));
                            }
                            0b0000001 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "divuw", rd, rs1, rs2));
                            }
                            _ => (),
                        },
                        6 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "remw", rd, rs1, rs2));
                        }
                        7 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "remuw", rd, rs1, rs2));
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            RiscInstruction::IType(i_type) => {
                let rs1 = find_register_name(i_type.rs1).unwrap();
                let rd = find_register_name(i_type.rd).unwrap();
                match i_type.op {
                    OPCODE_IMM => match i_type.funct3 {
                        0 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "addi", rd, rs1, i_type.imm));
                        }
                        1 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}",
                                "slli",
                                rd,
                                rs1,
                                i_type.imm & 0x003f
                            ));
                        }
                        2 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}",
                                "slti",
                                rd,
                                rs1,
                                i_type.imm & 0x003f
                            ));
                        }
                        3 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}",
                                "sltiu",
                                rd,
                                rs1,
                                i_type.imm & 0x003f
                            ));
                        }
                        4 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "xori", rd, rs1, i_type.imm));
                        }
                        5 => match i_type.imm {
                            0b000000 => {
                                string_version.push_str(&format!(
                                    "{} {}, {}, {}",
                                    "srli",
                                    rd,
                                    rs1,
                                    i_type.imm & 0x003f
                                ));
                            }
                            0b010000 => {
                                string_version.push_str(&format!(
                                    "{} {}, {}, {}",
                                    "srai",
                                    rd,
                                    rs1,
                                    i_type.imm & 0x003f
                                ));
                            }

                            _ => (),
                        },
                        6 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "ori", rd, rs1, i_type.imm));
                        }
                        7 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "andi", rd, rs1, i_type.imm));
                        }
                        _ => (),
                    },
                    OPCODE_IMM_32 => {
                        match i_type.funct3 {
                            0 => {
                                string_version.push_str(&format!(
                                    "{} {}, {}, {}",
                                    "addiw", rd, rs1, i_type.imm
                                ));
                            }
                            1 => {
                                // shift lower 5 bits of imm
                                string_version.push_str(&format!(
                                    "{} {}, {}, {}",
                                    "slliw",
                                    rd,
                                    rs1,
                                    i_type.imm & 0x001f
                                ));
                            }
                            5 => {
                                // match first 5 bits of imm
                                let imm = i_type.imm >> 5;
                                match imm {
                                    0b000000 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "srliw",
                                            rd,
                                            rs1,
                                            i_type.imm & 0x001f
                                        ));
                                    }
                                    0b010000 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "sraiw",
                                            rd,
                                            rs1,
                                            i_type.imm & 0x001f
                                        ));
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    }
                    OPCODE_JALR => {
                        string_version
                            .push_str(&format!("{} {}, {}, {}", "jalr", rd, rs1, i_type.imm));
                    }
                    OPCODE_LOAD => match i_type.funct3 {
                        0 => {
                            string_version.push_str(&format!("{} {}, offset({})", "lb", rd, rs1));
                        }
                        1 => {
                            string_version.push_str(&format!("{} {}, offset({})", "lh", rd, rs1));
                        }
                        2 => {
                            string_version.push_str(&format!("{} {}, offset({})", "lw", rd, rs1));
                        }
                        3 => {
                            string_version.push_str(&format!("{} {}, offset({})", "ld", rd, rs1));
                        }
                        4 => {
                            string_version.push_str(&format!("{} {}, offset({})", "lbu", rd, rs1));
                        }
                        5 => {
                            string_version.push_str(&format!("{} {}, offset({})", "lhu", rd, rs1));
                        }
                        6 => {
                            string_version.push_str(&format!("{} {}, offset({})", "lwu", rd, rs1));
                        }
                        _ => (),
                    },
                    OPCODE_SYSTEM => match i_type.funct3 {
                        0 => match i_type.imm {
                            0 => {
                                string_version.push_str("ecall");
                            }
                            1 => {
                                string_version.push_str("ebreak");
                            }
                            0b000000000010 => {
                                string_version.push_str("uret");
                            }
                            0b000100000010 => {
                                string_version.push_str("sret");
                            }
                            0b001100000010 => {
                                string_version.push_str("mret");
                            }
                            0b0001000001010 => {
                                string_version.push_str("wfi");
                            }
                            _ => (),
                        },
                        2 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "csrrw", rd, rs1, i_type.imm));
                        }
                        3 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "csrrs", rd, rs1, i_type.imm));
                        }
                        4 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "csrrc", rd, rs1, i_type.imm));
                        }
                        5 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "csrrwi", rd, rs1, i_type.imm));
                        }
                        6 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "csrrsi", rd, rs1, i_type.imm));
                        }
                        7 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "csrrci", rd, rs1, i_type.imm));
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            RiscInstruction::SType(s_type) => {
                let rs1 = find_register_name(s_type.rs1).unwrap();
                let rs2 = find_register_name(s_type.rs2).unwrap();
                if s_type.op == OPCODE_STORE {
                    match s_type.funct3 {
                        0 => {
                            string_version.push_str(&format!("{} {},  offset({})", "sb", rs2, rs1));
                        }
                        1 => {
                            string_version.push_str(&format!("{} {},  offset({})", "sh", rs2, rs1));
                        }
                        2 => {
                            string_version.push_str(&format!("{} {},  offset({})", "sw", rs2, rs1));
                        }
                        3 => {
                            string_version.push_str(&format!("{} {},  offset({})", "sd", rs2, rs1));
                        }
                        _ => (),
                    }
                }
            }
            RiscInstruction::BType(b_type) => {
                let rs1 = find_register_name(b_type.rs1).unwrap();
                let rs2 = find_register_name(b_type.rs2).unwrap();
                if b_type.op == OPCODE_BRANCH {
                    match b_type.funct3 {
                        0 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "beq", rs1, rs2, b_type.imm1));
                        }
                        1 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bne", rs1, rs2, b_type.imm1));
                        }
                        4 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "blt", rs1, rs2, b_type.imm1));
                        }
                        5 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bge", rs1, rs2, b_type.imm1));
                        }
                        6 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bltu", rs1, rs2, b_type.imm1));
                        }
                        7 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bgeu", rs1, rs2, b_type.imm1));
                        }
                        _ => (),
                    }
                }
            }
            RiscInstruction::UType(u_type) => {
                let rd = find_register_name(u_type.rd).unwrap();
                match u_type.op {
                    OPCODE_AUIPC => {
                        string_version.push_str(&format!("{} {}, {}", "auipc", rd, u_type.imm));
                    }
                    OPCODE_LUI => {
                        string_version.push_str(&format!("{} {}, {}", "lui", rd, u_type.imm));
                    }
                    _ => (),
                }
            }
            RiscInstruction::JType(j_type) => {
                let rd = find_register_name(j_type.rd).unwrap();
                if j_type.op == OPCODE_JAL {
                    string_version.push_str(&format!("{} {},  {}", "jal", rd, j_type.imm));
                }
            }
            _ => (),
        }
        Ok(string_version)
    }
}

pub fn find_register_name(binary: u8) -> Option<&'static str> {
    for register in RISCV_GP_REGISTERS {
        if register.binary == binary {
            // If a match is found, return the first name in the names array
            return Some(register.names[0]);
        }
    }
    // If no match is found, return None
    None
}
