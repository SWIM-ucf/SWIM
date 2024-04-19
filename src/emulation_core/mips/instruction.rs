//! Abstract representation of an instruction.

use std::collections::HashMap;

use crate::parser::parser_structs_and_enums::{FP_REGISTERS, GP_REGISTERS};
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
/// - funct: SYSCALL (`001100`), BREAK (`001101`)
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
pub enum MipsInstruction {
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

impl Default for MipsInstruction {
    fn default() -> Self {
        MipsInstruction::RType(RType::default())
    }
}

impl TryFrom<u32> for MipsInstruction {
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
                    FUNCT_SYSCALL => Ok(MipsInstruction::SyscallType(SyscallType {
                        op: ((value >> 26) & 0x3F) as u8,
                        code: ((value >> 6) & 0xFFFFF),
                        funct: (value & 0x3F) as u8,
                    })),
                    FUNCT_BREAK => Ok(MipsInstruction::SyscallType(SyscallType {
                        op: ((value >> 26) & 0x3F) as u8,
                        code: (value >> 6) & 0xFFFFF,
                        funct: (value & 0x3F) as u8,
                    })),
                    _ => Ok(MipsInstruction::RType(RType {
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
                                Ok(MipsInstruction::FpuRType(FpuRType {
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
                            | FUNCTION_C_NGT => {
                                Ok(MipsInstruction::FpuCompareType(FpuCompareType {
                                    op: ((value >> 26) & 0x3F) as u8,
                                    fmt: ((value >> 21) & 0x1F) as u8,
                                    ft: ((value >> 16) & 0x1F) as u8,
                                    fs: ((value >> 11) & 0x1F) as u8,
                                    cc: ((value >> 8) & 0x7) as u8,
                                    function: (value & 0x3F) as u8,
                                }))
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
                    SUB_MT | SUB_DMT | SUB_MF | SUB_DMF => {
                        Ok(MipsInstruction::FpuRegImmType(FpuRegImmType {
                            op: ((value >> 26) & 0x3F) as u8,
                            sub: ((value >> 21) & 0x1F) as u8,
                            rt: ((value >> 16) & 0x1F) as u8,
                            fs: ((value >> 11) & 0x1F) as u8,
                        }))
                    }

                    // Branch on coprocessor 1 true (bc1t)
                    // Branch on coprocessor 1 false (bc1f)
                    SUB_BC => Ok(MipsInstruction::FpuBranchType(FpuBranchType {
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
                Ok(MipsInstruction::IType(IType {
                    op: ((value >> 26) & 0x3F) as u8,
                    rs: ((value >> 21) & 0x1F) as u8,
                    rt: ((value >> 16) & 0x1F) as u8,
                    immediate: (value & 0xFFFF) as u16,
                }))
            }

            // Store/load word to Coprocessor 1
            OPCODE_SWC1 | OPCODE_LWC1 => Ok(MipsInstruction::FpuIType(FpuIType {
                op: ((value >> 26) & 0x3F) as u8,
                base: ((value >> 21) & 0x1F) as u8,
                ft: ((value >> 16) & 0x1F) as u8,
                offset: (value & 0xFFFF) as u16,
            })),

            OPCODE_J | OPCODE_JAL => Ok(MipsInstruction::JType(JType {
                op: ((value >> 26) & 0x3F) as u8,
                addr: value & 0x03ffffff,
            })),

            _ => Err(format!("opcode `{op}` not supported")),
        }
    }
}

impl MipsInstruction {
    pub fn get_string_version(
        value: u32,
        labels: HashMap<String, usize>,
        instruction_number: usize,
    ) -> Result<String, String> {
        let struct_representation = match MipsInstruction::try_from(value) {
            Ok(struct_representation) => struct_representation,
            Err(_) => return Ok("nop".to_string()),
        };
        let mut string_version = String::new();

        log::debug!("struct_representation: {:?}", struct_representation);
        // log all the fields of the struct_representation


        match struct_representation {
            MipsInstruction::RType(r_type) => {
                // R-type instructions:
                // add, sub, mul, div
                // addu
                // dadd, dsub, dmul, ddiv
                // daddu, dsubu, dmulu, ddivu
                // or, and, sll
                // slt, sltu
                // jalr, jr

                let str_rs = find_register_name(r_type.rs).unwrap_or("##");
                let str_rt = find_register_name(r_type.rt).unwrap_or("##");
                let str_rd = find_register_name(r_type.rd).unwrap_or("##");
                let shamt_binary_str = format!("{:?}", r_type.shamt);
                let str_shamt = shamt_binary_str.as_str();

                match r_type.op {
                    OPCODE_SPECIAL => match r_type.funct {
                        FUNCT_SYSCALL => {
                            string_version.push_str("syscall");
                        }
                        FUNCT_BREAK => {
                            string_version.push_str("break");
                        }
                        FUNCT_ADD => {
                            string_version
                                .push_str(&format!("add {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_ADDU => {
                            string_version
                                .push_str(&format!("addu {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_DADD => {
                            string_version
                                .push_str(&format!("dadd {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_DADDU => {
                            string_version
                                .push_str(&format!("daddu {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_SUB => {
                            string_version
                                .push_str(&format!("sub {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_DSUB => {
                            string_version
                                .push_str(&format!("dsub {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_DSUBU => {
                            string_version
                                .push_str(&format!("dsubu {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_AND => {
                            string_version
                                .push_str(&format!("and {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_OR => {
                            string_version
                                .push_str(&format!("or {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_SLL => {
                            // if all the fields are 0, then it is a nop
                            if r_type.rd == 0 && r_type.rt == 0 && r_type.shamt == 0 {
                                string_version.push_str("nop");
                            } else {
                                string_version
                                    .push_str(&format!("sll {}, {}, {}", str_rd, str_rt, str_shamt));
                            }
                        }
                        FUNCT_SLT => {
                            string_version
                                .push_str(&format!("slt {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_SLTU => {
                            string_version
                                .push_str(&format!("sltu {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_SOP30 => match r_type.shamt {
                            ENC_MUL => {
                                string_version
                                    .push_str(&format!("mul {}, {}, {}", str_rd, str_rs, str_rt));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        },
                        FUNCT_SOP31 => match r_type.shamt {
                            ENC_MULU => {
                                string_version
                                    .push_str(&format!("mulu {}, {}, {}", str_rd, str_rs, str_rt));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        },
                        FUNCT_SOP32 => {
                            string_version
                                .push_str(&format!("div {}, {}, {}", str_rd, str_rs, str_rt));
                        }
                        FUNCT_SOP33 => match r_type.shamt {
                            ENC_DIVU => {
                                string_version
                                    .push_str(&format!("divu {}, {}, {}", str_rd, str_rs, str_rt));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        },
                        FUNCT_SOP34 => match r_type.shamt {
                            ENC_DMUL => {
                                string_version
                                    .push_str(&format!("dmul {}, {}, {}", str_rd, str_rs, str_rt));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        },
                        FUNCT_SOP35 => match r_type.shamt {
                            ENC_DMULU => {
                                string_version
                                    .push_str(&format!("dmulu {}, {}, {}", str_rd, str_rs, str_rt));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        },
                        FUNCT_SOP36 => match r_type.shamt {
                            ENC_DIV => {
                                string_version
                                    .push_str(&format!("ddiv {}, {}, {}", str_rd, str_rs, str_rt));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        },
                        FUNCT_SOP37 => match r_type.shamt {
                            ENC_DIVU => {
                                string_version
                                    .push_str(&format!("ddivu {}, {}, {}", str_rd, str_rs, str_rt));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        },
                        FUNCT_JR => {
                            string_version.push_str(&format!("jr {}", str_rs));
                        }
                        _ => {
                            string_version.push_str("###");
                        }
                    },
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::IType(i_type) => {
                // I-Type instructions:
                // addi, addiu, daddi, daddiu
                // lw, sw
                // lui
                // ori, andi
                // regimm
                // beq, bne

                let str_rs = find_register_name(i_type.rs).unwrap_or("##");
                let str_rt = find_register_name(i_type.rt).unwrap_or("##");

                // Check if immediate is negative
                let mut str_immediate = format!("{}", i_type.immediate);
                if i_type.immediate & 0x800 != 0 {
                    str_immediate = format!(
                        "-{}",
                        (!(i_type.immediate) + 1) & 0b00000000000000000000111111111111
                    );
                }

                match i_type.op {
                    OPCODE_ADDI => {
                        string_version
                            .push_str(&format!("addi {}, {}, {}", str_rt, str_rs, str_immediate));
                    }
                    OPCODE_ADDIU => {
                        string_version
                            .push_str(&format!("addiu {}, {}, {}", str_rt, str_rs, str_immediate));
                    }
                    OPCODE_DADDI => {
                        string_version
                            .push_str(&format!("daddi {}, {}, {}", str_rt, str_rs, str_immediate));
                    }
                    OPCODE_DADDIU => {
                        string_version
                            .push_str(&format!("daddiu {}, {}, {}", str_rt, str_rs, str_immediate));
                    }
                    OPCODE_LW => {
                        string_version
                            .push_str(&format!("lw {}, {}({})", str_rt, str_immediate, str_rs));
                    }
                    OPCODE_SW => {
                        string_version
                            .push_str(&format!("sw {}, {}({})", str_rt, str_immediate, str_rs));
                    }
                    OPCODE_LUI => {
                        let str_immediate = i_type.immediate as u32;
                        string_version.push_str(&format!("lui {}, 0x{:x}", str_rt, str_immediate));
                    }
                    OPCODE_ORI => {
                        string_version
                            .push_str(&format!("ori {}, {}, {}", str_rt, str_rs, str_immediate));
                    }
                    OPCODE_ANDI => {
                        string_version
                            .push_str(&format!("andi {}, {}, {}", str_rt, str_rs, str_immediate));
                    }
                    OPCODE_REGIMM => {
                        // rt field is used as the register immediate subcode
                        match i_type.rt {
                            RMSUB_DAHI => {
                                string_version
                                    .push_str(&format!("dahi {}, {}", str_rs, str_immediate));
                            }
                            RMSUB_DATI => {
                                string_version
                                    .push_str(&format!("dati {}, {}", str_rs, str_immediate));
                            }
                            _ => {
                                string_version.push_str("###");
                            }
                        }
                    }
                    OPCODE_BEQ => {
                        let mut str_label = String::new();
                        for label in labels {
                            if label.1 == (i_type.immediate as usize) + instruction_number * 4 {
                                str_label = label.0;
                            }
                        }
                        string_version
                            .push_str(&format!("beq {}, {}, {}", str_rs, str_rt, str_label));
                    }
                    OPCODE_BNE => {
                        let mut str_label = String::new();
                        for label in labels {
                            if label.1 == (i_type.immediate as usize) + instruction_number * 4 {
                                str_label = label.0;
                            }
                        }
                        string_version
                            .push_str(&format!("bne {}, {}, {}", str_rs, str_rt, str_label));
                    }
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::JType(j_type) => {
                // J-Type instructions:
                // j, jal
                let addr = j_type.addr;

                let mut str_addr = addr.to_string();

                for label in labels {
                    if label.1 == (addr as usize) * 4 {
                        str_addr = label.0;
                    }
                }

                match j_type.op {
                    OPCODE_J => {
                        string_version.push_str(&format!("j {}", str_addr));
                    }
                    OPCODE_JAL => {
                        string_version.push_str(&format!("jal {}", str_addr));
                    }
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::SyscallType(syscall_type) => {
                // Syscall ("System Call") Instruction
                match syscall_type.funct {
                    FUNCT_SYSCALL => {
                        string_version.push_str("syscall");
                    }
                    // FUNCT_BREAK => {
                    //     string_version.push_str("break");
                    // }
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::FpuRType(fpu_r_type) => {
                // FPU R-Type instructions:
                // add.fmt, sub.fmt, mul.fmt, div.fmt

                let str_fs = find_register_name_fp(fpu_r_type.fs).unwrap_or("##");
                let str_ft = find_register_name_fp(fpu_r_type.ft).unwrap_or("##");
                let str_fd = find_register_name_fp(fpu_r_type.fd).unwrap_or("##");

                match fpu_r_type.fmt {
                    FMT_SINGLE => match fpu_r_type.function {
                        FUNCTION_ADD => {
                            string_version
                                .push_str(&format!("add.s {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        FUNCTION_SUB => {
                            string_version
                                .push_str(&format!("sub.s {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        FUNCTION_MUL => {
                            string_version
                                .push_str(&format!("mul.s {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        FUNCTION_DIV => {
                            string_version
                                .push_str(&format!("div.s {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        _ => {
                            string_version.push_str("###");
                        }
                    },
                    FMT_DOUBLE => match fpu_r_type.function {
                        FUNCTION_ADD => {
                            string_version
                                .push_str(&format!("add.d {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        FUNCTION_SUB => {
                            string_version
                                .push_str(&format!("sub.d {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        FUNCTION_MUL => {
                            string_version
                                .push_str(&format!("mul.d {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        FUNCTION_DIV => {
                            string_version
                                .push_str(&format!("div.d {}, {}, {}", str_fd, str_fs, str_ft));
                        }
                        _ => {
                            string_version.push_str("###");
                        }
                    },
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::FpuIType(fpu_i_type) => {
                // FPU I-Type instructions:
                // swc1, lwc1
                let str_base = find_register_name(fpu_i_type.base).unwrap_or("##"); // base is a GPRegister
                let str_ft = find_register_name_fp(fpu_i_type.ft).unwrap_or("##");
                let str_offset = fpu_i_type.offset.to_string();
                let str_offset = str_offset.as_str();

                match fpu_i_type.op {
                    OPCODE_SWC1 => {
                        string_version
                            .push_str(&format!("swc1 {}, {}({})", str_ft, str_offset, str_base));
                    }
                    OPCODE_LWC1 => {
                        string_version
                            .push_str(&format!("lwc1 {}, {}({})", str_ft, str_offset, str_base));
                    }
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::FpuRegImmType(fpu_reg_imm_type) => {
                // FPU Register-Immediate instructions:
                // mtc1, dmtc1, mfc1, dmfc1

                let str_rt = find_register_name(fpu_reg_imm_type.rt).unwrap_or("##");
                let str_fs = find_register_name_fp(fpu_reg_imm_type.fs).unwrap_or("##");

                match fpu_reg_imm_type.sub {
                    SUB_MT => {
                        string_version.push_str(&format!("mtc1 {}, {}", str_rt, str_fs));
                    }
                    SUB_DMT => {
                        string_version.push_str(&format!("dmtc1 {}, {}", str_rt, str_fs));
                    }
                    SUB_MF => {
                        string_version.push_str(&format!("mfc1 {}, {}", str_rt, str_fs));
                    }
                    SUB_DMF => {
                        string_version.push_str(&format!("dmfc1 {}, {}", str_rt, str_fs));
                    }
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::FpuBranchType(fpu_branch_type) => {
                // FPU Branching instructions:
                // bc1t, bc1f
                // TODO add support for labels

                match fpu_branch_type.bcc1 {
                    SUB_BC => {
                        let str_offset = fpu_branch_type.offset.to_string();
                        let str_offset = str_offset.as_str();

                        match fpu_branch_type.tf {
                            1 => {
                                string_version.push_str(&format!("bc1t {}", str_offset));
                            }
                            _ => {
                                string_version.push_str(&format!("bc1f {}", str_offset));
                            }
                        }
                    }
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
            MipsInstruction::FpuCompareType(fpu_compare_type) => {
                // FPU Comparison instructions:
                // c.eq.fmt, c.lt.fmt, c.le.fmt, c.ngt.fmt, c.nge.fmt

                let str_fs = find_register_name_fp(fpu_compare_type.fs).unwrap_or("##");
                let str_ft = find_register_name_fp(fpu_compare_type.ft).unwrap_or("##");

                match fpu_compare_type.fmt {
                    FMT_SINGLE => match fpu_compare_type.function {
                        FUNCTION_C_EQ => {
                            string_version.push_str(&format!("c.eq.s {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_LT => {
                            string_version.push_str(&format!("c.lt.s {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_NGE => {
                            string_version.push_str(&format!("c.nge.s {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_LE => {
                            string_version.push_str(&format!("c.le.s {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_NGT => {
                            string_version.push_str(&format!("c.ngt.s {}, {}", str_fs, str_ft));
                        }
                        _ => {
                            string_version.push_str("###");
                        }
                    },
                    FMT_DOUBLE => match fpu_compare_type.function {
                        FUNCTION_C_EQ => {
                            string_version.push_str(&format!("c.eq.d {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_LT => {
                            string_version.push_str(&format!("c.lt.d {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_NGE => {
                            string_version.push_str(&format!("c.nge.d {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_LE => {
                            string_version.push_str(&format!("c.le.d {}, {}", str_fs, str_ft));
                        }
                        FUNCTION_C_NGT => {
                            string_version.push_str(&format!("c.ngt.d {}, {}", str_fs, str_ft));
                        }
                        _ => {
                            string_version.push_str("###");
                        }
                    },
                    _ => {
                        string_version.push_str("###");
                    }
                }
            }
        }
        Ok(string_version)
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

pub fn find_register_name_fp(binary: u8) -> Option<&'static str> {
    for register in FP_REGISTERS {
        if register.binary == binary {
            // If a match is found, return the first name in the names array
            return Some(register.name);
        }
    }
    // If no match is found, return None
    None
}
