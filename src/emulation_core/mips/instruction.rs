//! Abstract representation of an instruction.

use crate::parser::parser_structs_and_enums::GP_REGISTERS;
use serde::{Deserialize, Serialize};

use super::constants::*;

/// Register (R-Type) Instruction
///
/// ```text
/// 31           26   25       21   20       16   15       11   10        6   5           0
/// ┌───────────────┬─────────────┬─────────────┬─────────────┬─────────────┬───────────────┐
/// │    opcode     │     rs      │     rt      │     rd      │    shamt    │   function    │
/// │               │             │             │             │             │               │
/// └───────────────┴─────────────┴─────────────┴─────────────┴─────────────┴───────────────┘
///         6              5             5             5             5              6
/// ```
///
/// - opcode: Determines the type of instruction executed. This is typically 000000 in R-type instructions.
/// - rs: CPU register - used as a source to read from in the register file.
/// - rt: CPU register - used as a source to read from in the register file.
/// - rd: CPU register - can be used as a destination for the result of executed instructions.
/// - shamt: Shift amount. Also called "shamt". Determines the amount of bits to shift in those instructions
///   that shift bits. Depending on the instruction, this field may be repurposed as a tertiary field for
///   determining the type of instruction executed (in `mul`, `dmul`, `dmulu`, `div`, `ddiv`,
///   `ddivu`), or be used as a "hint" field for certain instructions (of note are `jr` and `jalr`).
/// - function: Secondary field for determining the type of instruction executed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub shamt: u8,
    pub funct: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub immediate: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JType {
    pub op: u8,
    pub addr: u32,
}

/// Syscall ("System Call") Instruction
///
/// ```text
/// 31              26   25                                           6   5              0
/// ┌──────────────────┬────────────────────────────────────────────────┬──────────────────┐
/// │ opcode = SPECIAL │                     code                       │  funct = SYSCALL │
/// │      000000      │                                                │      001100      │
/// └──────────────────┴────────────────────────────────────────────────┴──────────────────┘
///         6                                 20                                  6
/// ```
///
/// - opcode: SPECIAL (`000000`)
/// - code: Available for use as software parameters.
/// - funct: SYSCALL (`001100`)
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SyscallType {
    pub op: u8,
    pub code: u32,
    pub funct: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FpuRType {
    pub op: u8,
    pub fmt: u8,
    pub ft: u8,
    pub fs: u8,
    pub fd: u8,
    pub function: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FpuIType {
    pub op: u8,
    pub base: u8,
    pub ft: u8,
    pub offset: u16,
}

/// Register-Immediate FPU Instruction
///
/// Used for instructions that transfer data between the main processor
/// and the floating-point coprocessor.
///
/// ```text
/// 31           26   25       21   20       16   15       11   10                    0
/// ┌───────────────┬─────────────┬─────────────┬─────────────┬─────────────────────────┐
/// │ opcode = COP1 │     sub     │     rt      │     fs      │            0            │
/// │    010001     │             │             │             │                         │
/// └───────────────┴─────────────┴─────────────┴─────────────┴─────────────────────────┘
///         6              5             5             5                   11
/// ```
///
/// - opcode: COP1 (`010001`)
/// - sub: Operation subcode field for COP1 register immediate-mode instructions.
/// - rt: CPU register - can be either source or destination.
/// - fs: FPU register - can be either source or destination.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FpuRegImmType {
    pub op: u8,
    pub sub: u8,
    pub rt: u8,
    pub fs: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FpuCompareType {
    pub op: u8,
    pub fmt: u8,
    pub ft: u8,
    pub fs: u8,
    pub cc: u8,
    pub function: u8,
}

/// Branching FPU Instruction
///
/// Used for instructions that branch based on the floating-point coprocessor.
///
/// ```text
/// 31           26   25       21   20    18   17   16   15                              0
/// ┌───────────────┬─────────────┬──────────┬────┬────┬──────────────────────────────────┐
/// │ opcode = COP1 │    BCC1     │    cc    │ nd │ tf │              offset              │
/// │    010001     │             │          │    │    │                                  │
/// └───────────────┴─────────────┴──────────┴────┴────┴──────────────────────────────────┘
///        6              5            3       1    1                   16
/// ```
///
/// - opcode: COP1 (`010001`)
/// - BCC1: "Branch conditional coprocessor 1" subcode.
/// - cc: Branch instruction condition code.
/// - nd: Nullify delay. If set, the branch is Likely, and the delay slot instruction is not executed. (Not necessary for this project.)
/// - tf: True/False. The type of condition for a comparison.
/// - offset: Signed offset field used in address calculations.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FpuBranchType {
    pub op: u8,
    pub bcc1: u8,
    pub cc: u8,
    pub nd: u8,
    pub tf: u8,
    pub offset: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    RType(RType),
    IType(IType),
    JType(JType),
    SyscallType(SyscallType),
    FpuRType(FpuRType),
    FpuIType(FpuIType),
    FpuRegImmType(FpuRegImmType),
    FpuCompareType(FpuCompareType),
    FpuBranchType(FpuBranchType),
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
        let op = (value >> 26) as u8;
        match op {
            // R-type instructions:
            // add, sub, mul, div
            // addu
            // dadd, dsub, dmul, ddiv
            // daddu, dsubu, dmulu, ddivu
            // or, and, sll
            // slt, sltu
            // jalr, jr
            //
            // Includes syscall.
            OPCODE_SPECIAL => {
                let funct = (value & 0x3F) as u8;

                match funct {
                    FUNCT_SYSCALL => Ok(Instruction::SyscallType(SyscallType {
                        op: ((value >> 26) & 0x3F) as u8,
                        code: ((value >> 6) & 0xFFFFF),
                        funct: (value & 0x3F) as u8,
                    })),
                    _ => Ok(Instruction::RType(RType {
                        op: ((value >> 26) & 0x3F) as u8,
                        rs: ((value >> 21) & 0x1F) as u8,
                        rt: ((value >> 16) & 0x1F) as u8,
                        rd: ((value >> 11) & 0x1F) as u8,
                        shamt: ((value >> 6) & 0x1F) as u8,
                        funct: (value & 0x3F) as u8,
                    })),
                }
            }

            // COP1 (coprocessor 1)
            OPCODE_COP1 => {
                // First break down the instruction by its `fmt`/`rs`/`bcc1` field.
                // Also called `sub` (operation subcode) field.
                let sub = ((value >> 21) & 0x1F) as u8;

                match sub {
                    // If it is the "s" or "d" fmts, use the `function` field.
                    FMT_SINGLE | FMT_DOUBLE => {
                        let function = (value & 0x3F) as u8;
                        match function {
                            // add.fmt, sub.fmt, mul.fmt, div.fmt
                            FUNCTION_ADD | FUNCTION_SUB | FUNCTION_MUL | FUNCTION_DIV => {
                                Ok(Instruction::FpuRType(FpuRType {
                                    op: ((value >> 26) & 0x3F) as u8,
                                    fmt: ((value >> 21) & 0x1F) as u8,
                                    ft: ((value >> 16) & 0x1F) as u8,
                                    fs: ((value >> 11) & 0x1F) as u8,
                                    fd: ((value >> 6) & 0x1F) as u8,
                                    function: (value & 0x3F) as u8,
                                }))
                            }
                            // Comparison instructions:
                            // c.eq.fmt, c.lt.fmt, c.le.fmt, c.ngt.fmt, c.nge.fmt
                            FUNCTION_C_EQ | FUNCTION_C_LT | FUNCTION_C_NGE | FUNCTION_C_LE
                            | FUNCTION_C_NGT => Ok(Instruction::FpuCompareType(FpuCompareType {
                                op: ((value >> 26) & 0x3F) as u8,
                                fmt: ((value >> 21) & 0x1F) as u8,
                                ft: ((value >> 16) & 0x1F) as u8,
                                fs: ((value >> 11) & 0x1F) as u8,
                                cc: ((value >> 8) & 0x7) as u8,
                                function: (value & 0x3F) as u8,
                            })),
                            _ => Err(format!(
                                "function `{function}` not supported for opcode {op}"
                            )),
                        }
                    }

                    // Move word to coprocessor 1 (mtc1)
                    // Move doubleword to coprocessor 1 (dmtc1)
                    // Move word from coprocessor 1 (mfc1)
                    // Move doubleword from coprocessor 1 (dmfc1)
                    SUB_MT | SUB_DMT | SUB_MF | SUB_DMF => {
                        Ok(Instruction::FpuRegImmType(FpuRegImmType {
                            op: ((value >> 26) & 0x3F) as u8,
                            sub: ((value >> 21) & 0x1F) as u8,
                            rt: ((value >> 16) & 0x1F) as u8,
                            fs: ((value >> 11) & 0x1F) as u8,
                        }))
                    }

                    // Branch on coprocessor 1 true (bc1t)
                    // Branch on coprocessor 1 false (bc1f)
                    SUB_BC => Ok(Instruction::FpuBranchType(FpuBranchType {
                        op: ((value >> 26) & 0x3F) as u8,
                        bcc1: ((value >> 21) & 0x1F) as u8,
                        cc: ((value >> 18) & 0x7) as u8,
                        nd: ((value >> 17) & 1) as u8,
                        tf: ((value >> 16) & 1) as u8,
                        offset: (value & 0xFFFF) as u16,
                    })),

                    _ => Err(format!("sub code `{sub}` not supported for opcode {op}")),
                }
            }

            // I-Type instructions:
            OPCODE_ADDI | OPCODE_ADDIU | OPCODE_DADDI | OPCODE_DADDIU | OPCODE_LW | OPCODE_SW
            | OPCODE_LUI | OPCODE_ORI | OPCODE_ANDI | OPCODE_REGIMM | OPCODE_BEQ | OPCODE_BNE => {
                Ok(Instruction::IType(IType {
                    op: ((value >> 26) & 0x3F) as u8,
                    rs: ((value >> 21) & 0x1F) as u8,
                    rt: ((value >> 16) & 0x1F) as u8,
                    immediate: (value & 0xFFFF) as u16,
                }))
            }

            // Store/load word to Coprocessor 1
            OPCODE_SWC1 | OPCODE_LWC1 => Ok(Instruction::FpuIType(FpuIType {
                op: ((value >> 26) & 0x3F) as u8,
                base: ((value >> 21) & 0x1F) as u8,
                ft: ((value >> 16) & 0x1F) as u8,
                offset: (value & 0xFFFF) as u16,
            })),

            OPCODE_J | OPCODE_JAL => Ok(Instruction::JType(JType {
                op: ((value >> 26) & 0x3F) as u8,
                addr: value & 0x03ffffff,
            })),

            _ => Err(format!("opcode `{op}` not supported")),
        }
    }
}

pub fn find_register_name(binary: u8) -> Option<&'static str> {
    for register in GP_REGISTERS {
        if register.binary == binary {
            // If a match is found, return the first name in the names array
            return Some(register.names[0]);
        }
    }
    // If no match is found, return None
    None
}

pub fn get_string_version(value: u32) -> Result<String, String> {
    let mut string_version = String::new();
    let op = (value >> 26) as u8;
    let rs = ((value >> 21) & 0x1F) as u8;
    let rt = ((value >> 16) & 0x1F) as u8;
    let rd = ((value >> 11) & 0x1F) as u8;
    let mut immediate = (value & 0xFFFF) as u16;
    let mut imm_with_sign: i32 = 0;

    let mut imm_is_negative = false;

    if op == 0 && rs == 0 && rt == 0 && rd == 0 && immediate == 0 {
        return Err(String::from("empty instruction"));
    }

    if value & 0xF000 > 0 {
        imm_is_negative = true;
        immediate = !(immediate) + 1;
        imm_with_sign = -(immediate as i32);
    }

    let str_rt = find_register_name(rt).unwrap_or("##");
    let str_rs = find_register_name(rs).unwrap_or("##");
    let str_rd = find_register_name(rd).unwrap_or("##");
    let mut string_imm = immediate.to_string();
    let mut str_immediate = string_imm.as_str();

    if imm_is_negative {
        string_imm = imm_with_sign.to_string();
        str_immediate = string_imm.as_str();
    }

    let str_ft = str_rt;
    let str_base = str_rs;
    let str_offset = str_immediate;

    let funct = (value & 0x3F) as u8;
    let shamt = (value & 0x3F) as u8;
    let shamt_binary_str = format!("{:b}", shamt);
    let str_shamt = shamt_binary_str.as_str();

    match op {
        // R-type instructions:
        // add, sub, mul, div
        // addu
        // dadd, dsub, dmul, ddiv
        // daddu, dsubu, dmulu, ddivu
        // or, and, sll
        // slt, sltu
        // jalr, jr
        //
        // Includes syscall.
        OPCODE_SPECIAL => {
            match funct {
                FUNCT_SYSCALL => Ok(String::from("syscall")),
                FUNCT_ADD => {
                    string_version = format!("{} {} {} {}", "add", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_ADDU => {
                    string_version = format!("{} {} {} {}", "addu", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_DADD => {
                    string_version = format!("{} {} {} {}", "dadd", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_DADDU => {
                    string_version = format!("{} {} {} {}", "daddu", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_SUB => {
                    string_version = format!("{} {} {} {}", "sub", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_DSUB => {
                    string_version = format!("{} {} {} {}", "dsub", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_DSUBU => {
                    string_version = format!("{} {} {} {}", "dsubu", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_AND => {
                    string_version = format!("{} {} {} {}", "and", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_OR => {
                    string_version = format!("{} {} {} {}", "or", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_SLL => {
                    string_version = format!("{} {} {} {}", "sll", str_rd, str_rs, str_shamt);
                    Ok(string_version)
                }
                FUNCT_SLT => {
                    string_version = format!("{} {} {} {}", "slt", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_SLTU => {
                    string_version = format!("{} {} {} {}", "sltu", str_rd, str_rs, str_shamt);
                    Ok(string_version)
                }
                FUNCT_SOP32 => {
                    string_version = format!("{} {} {} {}", "or", str_rd, str_rs, str_rt);
                    Ok(string_version)
                }
                FUNCT_SOP36 => match shamt {
                    // ENC_DIV == ENC_DDIV
                    ENC_DIV => {
                        string_version = format!("{} {} {}", "div", str_rs, str_rt);
                        Ok(string_version)
                    }
                    _ => {
                        string_version = String::from("###");
                        Ok(string_version)
                    }
                },
                FUNCT_SOP33 | FUNCT_SOP37 => match shamt {
                    // ENC_DIVU == ENC_DDIVU
                    ENC_DIVU => {
                        string_version = format!("{} {} {}", "divu", str_rs, str_rt);
                        Ok(string_version)
                    }
                    _ => {
                        string_version = String::from("###");
                        Ok(string_version)
                    }
                },
                FUNCT_SOP30 | FUNCT_SOP34 => match shamt {
                    // ENC_MUL == ENC_DMUL
                    ENC_MUL => {
                        string_version = format!("{} {} {} {}", "mul", str_rd, str_rs, str_rt);
                        Ok(string_version)
                    }
                    _ => {
                        string_version = String::from("###");
                        Ok(string_version)
                    }
                },
                FUNCT_SOP31 | FUNCT_SOP35 => match shamt {
                    // ENC_MULU == ENC_DMULU
                    ENC_MULU => {
                        string_version = format!("{} {} {} {}", "mulu", str_rd, str_rs, str_rt);
                        Ok(string_version)
                    }
                    _ => {
                        string_version = String::from("###");
                        Ok(string_version)
                    }
                },
                _ => {
                    string_version = String::from("###");
                    Ok(string_version)
                }
            }
        }

        // COP1 (coprocessor 1)
        OPCODE_COP1 => {
            // First break down the instruction by its `fmt`/`rs`/`bcc1` field.
            // Also called `sub` (operation subcode) field.
            let op = ((value >> 26) & 0x3F) as u8;
            let sub = ((value >> 21) & 0x1F) as u8;
            let ft = ((value >> 16) & 0x1F) as u8; // also rt
            let fs = ((value >> 11) & 0x1F) as u8; // also rs
            let fd = ((value >> 6) & 0x1F) as u8;
            let function = (value & 0x3F) as u8;
            let str_fs = find_register_name(fs).unwrap_or("##");
            let str_ft = find_register_name(ft).unwrap_or("##");
            let str_fd = find_register_name(fd).unwrap_or("##");

            match sub {
                // If it is the "s" or "d" fmts, use the `function` field.
                FMT_SINGLE => {
                    match function {
                        // add.fmt, sub.fmt, mul.fmt, div.fmt
                        FUNCTION_ADD => {
                            let string_version = format!("add.s {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_SUB => {
                            let string_version = format!("sub.s {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_MUL => {
                            let string_version = format!("mul.s {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_DIV => {
                            let string_version = format!("add.s {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        // Comparison instructions:
                        // c.eq.fmt, c.lt.fmt, c.le.fmt, c.ngt.fmt, c.nge.fmt
                        FUNCTION_C_EQ => {
                            let string_version = format!("c.eq.s {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_LT => {
                            let string_version = format!("c.lt.s {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_NGE => {
                            let string_version = format!("c.nge.s {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_LE => {
                            let string_version = format!("c.le.s {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_NGT => {
                            let string_version = format!("c.ngt.s {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        _ => Err(format!(
                            "function `{function}` not supported for opcode {op}"
                        )),
                    }
                }
                FMT_DOUBLE => {
                    match function {
                        // add.fmt, sub.fmt, mul.fmt, div.fmt
                        FUNCTION_ADD => {
                            let string_version = format!("add.d {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_SUB => {
                            let string_version = format!("sub.d {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_MUL => {
                            let string_version = format!("mul.d {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_DIV => {
                            let string_version = format!("add.d {} {} {}", str_fd, str_fs, str_ft);
                            Ok(string_version)
                        }
                        // Comparison instructions:
                        // c.eq.fmt, c.lt.fmt, c.le.fmt, c.ngt.fmt, c.nge.fmt
                        FUNCTION_C_EQ => {
                            let string_version = format!("c.eq.d {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_LT => {
                            let string_version = format!("c.lt.d {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_NGE => {
                            let string_version = format!("c.nge.d {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_LE => {
                            let string_version = format!("c.le.d {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        FUNCTION_C_NGT => {
                            let string_version = format!("c.ngt.d {} {}", str_fs, str_ft);
                            Ok(string_version)
                        }
                        _ => Err(format!(
                            "function `{function}` not supported for opcode {op}"
                        )),
                    }
                }

                // Move word to coprocessor 1 (mtc1)
                // Move doubleword to coprocessor 1 (dmtc1)
                // Move word from coprocessor 1 (mfc1)
                // Move doubleword from coprocessor 1 (dmfc1)
                SUB_MT => {
                    let string_version = format!("mtc1 {} {}", str_ft, str_fs);
                    Ok(string_version)
                }
                SUB_DMT => {
                    let string_version = format!("dmtc1 {} {}", str_ft, str_fs);
                    Ok(string_version)
                }
                SUB_MF => {
                    let string_version = format!("mfc1 {} {}", str_ft, str_fs);
                    Ok(string_version)
                }
                SUB_DMF => {
                    let string_version = format!("dfmc1 {} {}", str_ft, str_fs);
                    Ok(string_version)
                }

                // Branch on coprocessor 1 true (bc1t)
                // Branch on coprocessor 1 false (bc1f)
                SUB_BC => {
                    string_version.push_str("bc1t");
                    Ok(string_version)
                }

                _ => Err(format!("sub code `{sub}` not supported for opcode {op}")),
            }
        }

        // I-Type instructions:
        OPCODE_ADDI => {
            let string_version = format!("addi {} {} {}", str_rt, str_rs, str_immediate);
            Ok(string_version)
        }
        OPCODE_ADDIU => {
            let string_version = format!("addiu {} {} {}", str_rt, str_rs, str_immediate);
            Ok(string_version)
        }
        OPCODE_DADDI => {
            let string_version = format!("daddi {} {} {}", str_rt, str_rs, str_immediate);
            Ok(string_version)
        }
        OPCODE_DADDIU => {
            let string_version = format!("daddi {} {} {}", str_rt, str_rs, str_immediate);
            Ok(string_version)
        }
        OPCODE_LW => {
            let string_version = format!("lw {}, {} ({})", str_rt, str_immediate, str_rs);
            Ok(string_version)
        }
        OPCODE_SW => {
            let string_version = format!("sw {}, {} ({})", str_rt, str_immediate, str_rs);
            Ok(string_version)
        }
        OPCODE_LUI => {
            let string_version = format!("lui {} , {}", str_rt, str_immediate);
            Ok(string_version)
        }
        OPCODE_ORI => {
            let string_version = format!("ori {} {} {}", str_rt, str_rs, str_immediate);
            Ok(string_version)
        }
        OPCODE_ANDI => {
            let string_version = format!("andi {} {} {}", str_rt, str_rs, str_immediate);
            Ok(string_version)
        }
        OPCODE_REGIMM => {
            let string_version = format!(" dahi {} {}", str_rs, str_immediate);
            Ok(string_version)
        }
        OPCODE_BEQ => {
            let string_version = format!("beq {} {} {}", str_rs, str_rt, str_immediate);
            Ok(string_version)
        }
        OPCODE_BNE => {
            let string_version = format!("beq {} {} {}", str_rs, str_rt, str_immediate);
            Ok(string_version)
        }
        // Store/load word to Coprocessor 1
        OPCODE_SWC1 => {
            let string_version = format!("swc1 {} {} {}", str_ft, str_offset, str_base);
            Ok(string_version)
        }
        OPCODE_LWC1 => {
            let string_version = format!("lwc1 {} {} {}", str_ft, str_offset, str_base);
            Ok(string_version)
        }
        OPCODE_J => {
            let addr = value & 0x03ffffff;
            let str_addr = format!("{:b}", addr);
            let str_addr = str_addr.as_str();
            let string_version = format!("j {}", str_addr);
            Ok(string_version)
        }
        OPCODE_JAL => {
            let addr = value & 0x03ffffff;
            let str_addr = format!("{:b}", addr);
            let str_addr = str_addr.as_str();

            let string_version = format!("jal {}", str_addr);
            Ok(string_version)
        }

        _ => Err(format!("opcode `{op}` not supported")),
    }
}
