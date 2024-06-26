//! Abstract representation of an instruction.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::parser::parser_structs_and_enums::{RISCV_FP_REGISTERS, RISCV_GP_REGISTERS};

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
            OPCODE_OP | OPCODE_OP_32 | OPCODE_OP_FP => Ok(RiscInstruction::RType(RType {
                funct7: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // I-type instructions:
            OPCODE_IMM | OPCODE_IMM_32 | OPCODE_JALR | OPCODE_LOAD | OPCODE_SYSTEM
            | OPCODE_LOAD_FP => Ok(RiscInstruction::IType(IType {
                imm: (value >> 20) as u16,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // S-type instruction:
            OPCODE_STORE | OPCODE_STORE_FP => Ok(RiscInstruction::SType(SType {
                imm1: (value >> 25) as u8,
                rs2: ((value >> 20) & 0x1f) as u8,
                rs1: ((value >> 15) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                imm2: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // B-type instruction:
            OPCODE_BRANCH => Ok(RiscInstruction::BType(BType {
                imm: (value >> 20) as u16,
                rs1: ((value >> 7) & 0x1f) as u8,
                funct3: ((value >> 12) & 0x07) as u8,
                rs2: ((value >> 15) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // U-type instruction:
            OPCODE_LUI | OPCODE_AUIPC => Ok(RiscInstruction::UType(UType {
                imm: (value >> 12) as i32,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // J-type instruction:
            OPCODE_JAL => Ok(RiscInstruction::JType(JType {
                imm: (value >> 12) as i32,
                rd: ((value >> 7) & 0x1f) as u8,
                op: (value & 0x7f) as u8,
            })),

            // R4-type instruction:
            OPCODE_MADD | OPCODE_MSUB | OPCODE_NMSUB | OPCODE_NMADD => {
                Ok(RiscInstruction::R4Type(R4Type {
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

impl RiscInstruction {
    pub fn get_string_version(
        value: u32,
        labels: HashMap<String, usize>,
    ) -> Result<String, String> {
        let mut string_version = String::new();

        let struct_representation = RiscInstruction::try_from(value)?;

        match struct_representation {
            RiscInstruction::RType(r_type) => {
                let mut rs1 = find_register_name(r_type.rs1).unwrap();
                let mut rs2 = find_register_name(r_type.rs2).unwrap();
                let mut rd = find_register_name(r_type.rd).unwrap();
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
                    // RISCV64F
                    OPCODE_OP_FP => {
                        rd = find_register_name_fp(r_type.rd).unwrap();
                        rs1 = find_register_name_fp(r_type.rs1).unwrap();
                        rs2 = find_register_name_fp(r_type.rs2).unwrap();

                        match r_type.funct7 >> 2 {
                            0 => match r_type.funct7 & 0b11 {
                                0 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fadd.s", rd, rs1, rs2
                                    ));
                                }
                                1 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fadd.d", rd, rs1, rs2
                                    ));
                                }
                                _ => (),
                            },
                            1 => match r_type.funct7 & 0b11 {
                                0 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fsub.s", rd, rs1, rs2
                                    ));
                                }
                                1 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fsub.d", rd, rs1, rs2
                                    ));
                                }
                                _ => (),
                            },
                            2 => match r_type.funct7 & 0b11 {
                                0 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fmul.s", rd, rs1, rs2
                                    ));
                                }
                                1 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fmul.d", rd, rs1, rs2
                                    ));
                                }
                                _ => (),
                            },
                            3 => match r_type.funct7 & 0b11 {
                                0 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fdiv.s", rd, rs1, rs2
                                    ));
                                }
                                1 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fdiv.d", rd, rs1, rs2
                                    ));
                                }
                                _ => (),
                            },
                            4 => match r_type.funct3 {
                                0 => match r_type.funct7 & 0b11 {
                                    0 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fsgnj.s", rd, rs1, rs2
                                        ));
                                    }
                                    1 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fsgnj.d", rd, rs1, rs2
                                        ));
                                    }
                                    _ => (),
                                },
                                1 => match r_type.funct7 & 0b11 {
                                    0 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fsgnjn.s", rd, rs1, rs2
                                        ));
                                    }
                                    1 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fsgnjn.d", rd, rs1, rs2
                                        ));
                                    }
                                    _ => (),
                                },
                                2 => match r_type.funct7 & 0b11 {
                                    0 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fsgnjx.s", rd, rs1, rs2
                                        ));
                                    }
                                    1 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fsgnjx.d", rd, rs1, rs2
                                        ));
                                    }
                                    _ => (),
                                },
                                _ => (),
                            },
                            5 => match r_type.funct3 {
                                0 => match r_type.funct7 & 0b11 {
                                    0 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fmin.s", rd, rs1, rs2
                                        ));
                                    }
                                    1 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fmin.d", rd, rs1, rs2
                                        ));
                                    }
                                    _ => (),
                                },
                                1 => match r_type.funct7 & 0b11 {
                                    0 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fmax.s", rd, rs1, rs2
                                        ));
                                    }
                                    1 => {
                                        string_version.push_str(&format!(
                                            "{} {}, {}, {}",
                                            "fmax.d", rd, rs1, rs2
                                        ));
                                    }
                                    _ => (),
                                },
                                _ => (),
                            },
                            8 => match r_type.rs2 {
                                0 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fcvt.d.s", rd, rs1, rs2
                                    ));
                                }
                                1 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fcvt.s.d", rd, rs1, rs2
                                    ));
                                }
                                _ => (),
                            },
                            11 => match r_type.funct7 & 0b11 {
                                0 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fsqrt.s", rd, rs1, rs2
                                    ));
                                }
                                1 => {
                                    string_version.push_str(&format!(
                                        "{} {}, {}, {}",
                                        "fsqrt.d", rd, rs1, rs2
                                    ));
                                }
                                _ => (),
                            },
                            20 => {
                                rd = find_register_name(r_type.rd).unwrap();
                                match r_type.funct3 {
                                    0 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}, {}",
                                                "fle.s", rd, rs1, rs2
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}, {}",
                                                "fle.d", rd, rs1, rs2
                                            ));
                                        }
                                        _ => (),
                                    },
                                    1 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}, {}",
                                                "flt.s", rd, rs1, rs2
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}, {}",
                                                "flt.d", rd, rs1, rs2
                                            ));
                                        }
                                        _ => (),
                                    },
                                    2 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}, {}",
                                                "feq.s", rd, rs1, rs2
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}, {}",
                                                "feq.d", rd, rs1, rs2
                                            ));
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }
                            24 => {
                                rd = find_register_name(r_type.rd).unwrap();
                                match r_type.rs2 {
                                    0 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.w.s", rd, rs1
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.w.d", rd, rs1
                                            ));
                                        }
                                        _ => (),
                                    },
                                    1 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.wu.s", rd, rs1
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.wu.d", rd, rs1
                                            ));
                                        }
                                        _ => (),
                                    },
                                    2 => {
                                        string_version
                                            .push_str(&format!("{} {}, {}", "fcvt.l.s", rd, rs1));
                                    }
                                    3 => {
                                        string_version
                                            .push_str(&format!("{} {}, {}", "fcvt.lu.s", rd, rs1));
                                    }
                                    _ => (),
                                }
                            }
                            26 => {
                                rs1 = find_register_name(r_type.rs1).unwrap();
                                match r_type.rs2 {
                                    0 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.s.w", rd, rs1
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.d.w", rd, rs1
                                            ));
                                        }
                                        _ => (),
                                    },
                                    1 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.s.wu", rd, rs1
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fcvt.d.wu", rd, rs1
                                            ));
                                        }
                                        _ => (),
                                    },
                                    2 => {
                                        string_version
                                            .push_str(&format!("{} {}, {}", "fcvt.s.l", rd, rs1));
                                    }
                                    3 => {
                                        string_version
                                            .push_str(&format!("{} {}, {}", "fcvt.s.lu", rd, rs1));
                                    }
                                    _ => (),
                                }
                            }
                            28 => {
                                rd = find_register_name(r_type.rd).unwrap();
                                match r_type.funct3 {
                                    0 => {
                                        string_version
                                            .push_str(&format!("{} {}, {}", "fmv.x.w", rd, rs1));
                                    }
                                    1 => match r_type.funct7 & 0b11 {
                                        0 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fclass.s", rd, rs1
                                            ));
                                        }
                                        1 => {
                                            string_version.push_str(&format!(
                                                "{} {}, {}",
                                                "fclass.d", rd, rs1
                                            ));
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }
                            30 => {
                                rs1 = find_register_name(r_type.rs1).unwrap();
                                string_version.push_str(&format!("{} {}, {}", "fmv.w.x", rd, rs1));
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            RiscInstruction::IType(i_type) => {
                let rs1 = find_register_name(i_type.rs1).unwrap();
                let mut rd = find_register_name(i_type.rd).unwrap();
                let imm = i_type.imm as i32;

                // Check if immediate is negative
                let mut str_imm = format!("{}", imm);
                if imm & 0x800 != 0 {
                    str_imm = format!("-{}", (!(imm) + 1) & 0b00000000000000000000111111111111);
                }

                match i_type.op {
                    OPCODE_IMM => match i_type.funct3 {
                        0 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "addi", rd, rs1, str_imm));
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
                        // match first 6 bits of imm
                        5 => match i_type.imm >> 6 {
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
                                .push_str(&format!("{} {}, {}, {}", "andi", rd, rs1, str_imm));
                        }
                        _ => (),
                    },
                    OPCODE_IMM_32 => {
                        match i_type.funct3 {
                            0 => {
                                string_version
                                    .push_str(&format!("{} {}, {}, {}", "addiw", rd, rs1, str_imm));
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
                            string_version.push_str(&format!("{} {}, {}({})", "lb", rd, imm, rs1));
                        }
                        1 => {
                            string_version.push_str(&format!("{} {}, {}({})", "lh", rd, imm, rs1));
                        }
                        2 => {
                            string_version.push_str(&format!("{} {}, {}({})", "lw", rd, imm, rs1));
                        }
                        3 => {
                            string_version.push_str(&format!("{} {}, {}({})", "ld", rd, imm, rs1));
                        }
                        4 => {
                            string_version.push_str(&format!("{} {}, {}({})", "lbu", rd, imm, rs1));
                        }
                        5 => {
                            string_version.push_str(&format!("{} {}, {}({})", "lhu", rd, imm, rs1));
                        }
                        6 => {
                            string_version.push_str(&format!("{} {}, {}({})", "lwu", rd, imm, rs1));
                        }
                        _ => (),
                    },
                    OPCODE_LOAD_FP => {
                        rd = find_register_name_fp(i_type.rd).unwrap();
                        match i_type.funct3 {
                            2 => {
                                string_version
                                    .push_str(&format!("{} {}, {}({})", "flw", rd, imm, rs1));
                            }
                            3 => {
                                string_version
                                    .push_str(&format!("{} {}, {}({})", "fld", rd, imm, rs1));
                            }
                            _ => (),
                        }
                    }
                    OPCODE_SYSTEM => match i_type.funct3 {
                        0 => match i_type.imm {
                            0 => {
                                string_version.push_str("ecall");
                            }
                            1 => {
                                string_version.push_str("ebreak");
                            }
                            2 => {
                                string_version.push_str("uret");
                            }
                            0b000100000010 => {
                                string_version.push_str("sret");
                            }
                            0b001100000010 => {
                                string_version.push_str("mret");
                            }
                            0b000100000101 => {
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
                let mut rs2 = find_register_name(s_type.rs2).unwrap();
                let offset = (s_type.imm1 as i32) << 5 | s_type.imm2 as i32;
                match s_type.op {
                    OPCODE_STORE => match s_type.funct3 {
                        0 => {
                            string_version
                                .push_str(&format!("{} {}, {}({})", "sb", rs2, offset, rs1));
                        }
                        1 => {
                            string_version
                                .push_str(&format!("{} {}, {}({})", "sh", rs2, offset, rs1));
                        }
                        2 => {
                            string_version
                                .push_str(&format!("{} {}, {}({})", "sw", rs2, offset, rs1));
                        }
                        3 => {
                            string_version
                                .push_str(&format!("{} {}, {}({})", "sd", rs2, offset, rs1));
                        }
                        _ => (),
                    },
                    OPCODE_STORE_FP => {
                        rs2 = find_register_name_fp(s_type.rs2).unwrap();
                        match s_type.funct3 {
                            2 => {
                                string_version
                                    .push_str(&format!("{} {}, {}({})", "fsw", rs2, offset, rs1));
                            }
                            3 => {
                                string_version
                                    .push_str(&format!("{} {}, {}({})", "fsd", rs2, offset, rs1));
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            RiscInstruction::BType(b_type) => {
                let rs1 = find_register_name(b_type.rs1).unwrap();
                let rs2 = find_register_name(b_type.rs2).unwrap();
                if b_type.op == OPCODE_BRANCH {
                    let mut str_label = format!("0x{:x}", b_type.imm);

                    for label in labels {
                        if label.1 == (b_type.imm as usize) * 4 {
                            str_label = label.0;
                        }
                    }
                    match b_type.funct3 {
                        0 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "beq", rs1, rs2, str_label));
                        }
                        1 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bne", rs1, rs2, str_label));
                        }
                        4 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "blt", rs1, rs2, str_label));
                        }
                        5 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bge", rs1, rs2, str_label));
                        }
                        6 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bltu", rs1, rs2, str_label));
                        }
                        7 => {
                            string_version
                                .push_str(&format!("{} {}, {}, {}", "bgeu", rs1, rs2, str_label));
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
                // TODO get current address
                let rd = find_register_name(j_type.rd).unwrap();
                let mut str_label = format!("0x{:x}", j_type.imm.wrapping_shl(2));
                for label in labels {
                    if label.1 == (j_type.imm as usize) * 4 {
                        str_label = label.0;
                    }
                }
                if j_type.op == OPCODE_JAL {
                    string_version.push_str(&format!("{} {}, {}", "jal", rd, str_label));
                }
            }
            RiscInstruction::R4Type(r4_type) => {
                let rs1 = find_register_name_fp(r4_type.rs1).unwrap();
                let rs2 = find_register_name_fp(r4_type.rs2).unwrap();
                let rs3 = find_register_name_fp(r4_type.rs3).unwrap();
                let rd = find_register_name_fp(r4_type.rd).unwrap();
                match r4_type.op {
                    OPCODE_MADD => match r4_type.funct2 {
                        0 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fmadd.s", rd, rs1, rs2, rs3
                            ));
                        }
                        1 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fmadd.d", rd, rs1, rs2, rs3
                            ));
                        }
                        _ => (),
                    },
                    OPCODE_MSUB => match r4_type.funct2 {
                        0 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fmsub.s", rd, rs1, rs2, rs3
                            ));
                        }
                        1 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fmsub.d", rd, rs1, rs2, rs3
                            ));
                        }
                        _ => (),
                    },
                    OPCODE_NMSUB => match r4_type.funct2 {
                        0 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fnmsub.s", rd, rs1, rs2, rs3
                            ));
                        }
                        1 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fnmsub.d", rd, rs1, rs2, rs3
                            ));
                        }
                        _ => (),
                    },
                    OPCODE_NMADD => match r4_type.funct2 {
                        0 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fnmadd.s", rd, rs1, rs2, rs3
                            ));
                        }
                        1 => {
                            string_version.push_str(&format!(
                                "{} {}, {}, {}, {}",
                                "fnmadd.d", rd, rs1, rs2, rs3
                            ));
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }

        if string_version.is_empty() {
            Err(format!("instruction `{}` not supported", value))
        } else {
            Ok(string_version)
        }
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

pub fn find_register_name_fp(binary: u8) -> Option<&'static str> {
    for register in RISCV_FP_REGISTERS {
        if register.binary == binary {
            // If a match is found, return the first name in the names array
            return Some(register.names[0]);
        }
    }
    // If no match is found, return None
    None
}
