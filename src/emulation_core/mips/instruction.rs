//! Abstract representation of an instruction.

use super::constants::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub shamt: u8,
    pub funct: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub immediate: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JType {
    pub op: u8,
    pub addr: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FpuRType {
    pub op: u8,
    pub fmt: u8,
    pub ft: u8,
    pub fs: u8,
    pub fd: u8,
    pub function: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FpuRegImmType {
    pub op: u8,
    pub sub: u8,
    pub rt: u8,
    pub fs: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FpuCompareType {
    pub op: u8,
    pub fmt: u8,
    pub ft: u8,
    pub fs: u8,
    pub cc: u8,
    pub function: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    RType(RType),
    IType(IType),
    JType(JType),
    FpuRType(FpuRType),
    FpuIType(FpuIType),
    FpuRegImmType(FpuRegImmType),
    FpuCompareType(FpuCompareType),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::RType(RType::default())
    }
}

impl From<u32> for Instruction {
    /// Based on the opcode, convert a binary instruction into a struct representation.
    fn from(value: u32) -> Self {
        let op = (value >> 26) as u8;
        match op {
            // R-type instructions:
            // add, sub, mul, div
            // dadd, dsub, dmul, ddiv
            OPCODE_SPECIAL => Instruction::RType(RType {
                op: ((value >> 26) & 0x3F) as u8,
                rs: ((value >> 21) & 0x1F) as u8,
                rt: ((value >> 16) & 0x1F) as u8,
                rd: ((value >> 11) & 0x1F) as u8,
                shamt: ((value >> 6) & 0x1F) as u8,
                funct: (value & 0x3F) as u8,
            }),

            // COP1 (coprocessor 1)
            OPCODE_COP1 => {
                // First break down the instruction by its `fmt` or `rs` field.
                // Also called `sub` (operation subcode) field.
                let sub = ((value >> 21) & 0x1F) as u8;

                match sub {
                    // If it is the "s" or "d" fmts, use the `function` field.
                    FMT_SINGLE | FMT_DOUBLE => {
                        let function = (value & 0x3F) as u8;
                        match function {
                            // add.fmt, sub.fmt, mul.fmt, div.fmt
                            FUNCTION_ADD | FUNCTION_SUB | FUNCTION_MUL | FUNCTION_DIV => {
                                Instruction::FpuRType(FpuRType {
                                    op: ((value >> 26) & 0x3F) as u8,
                                    fmt: ((value >> 21) & 0x1F) as u8,
                                    ft: ((value >> 16) & 0x1F) as u8,
                                    fs: ((value >> 11) & 0x1F) as u8,
                                    fd: ((value >> 6) & 0x1F) as u8,
                                    function: (value & 0x3F) as u8,
                                })
                            }
                            // Comparison instructions:
                            // c.eq.fmt, c.lt.fmt, c.le.fmt, c.ngt.fmt, c.nge.fmt
                            FUNCTION_C_EQ | FUNCTION_C_LT | FUNCTION_C_NGE | FUNCTION_C_LE
                            | FUNCTION_C_NGT => Instruction::FpuCompareType(FpuCompareType {
                                op: ((value >> 26) & 0x3F) as u8,
                                fmt: ((value >> 21) & 0x1F) as u8,
                                ft: ((value >> 16) & 0x1F) as u8,
                                fs: ((value >> 11) & 0x1F) as u8,
                                cc: ((value >> 8) & 0x7) as u8,
                                function: (value & 0x3F) as u8,
                            }),
                            _ => unimplemented!(
                                "function `{}` not supported for opcode {}",
                                function,
                                op
                            ),
                        }
                    }

                    // Move word to coprocessor 1 (mtc1)
                    // Move doubleword to coprocessor 1 (dmtc1)
                    // Move word from coprocessor 1 (mfc1)
                    // Move doubleword from coprocessor 1 (dmfc1)
                    SUB_MT | SUB_DMT | SUB_MF | SUB_DMF => {
                        Instruction::FpuRegImmType(FpuRegImmType {
                            op: ((value >> 26) & 0x3F) as u8,
                            sub: ((value >> 21) & 0x1F) as u8,
                            rt: ((value >> 16) & 0x1F) as u8,
                            fs: ((value >> 11) & 0x1F) as u8,
                        })
                    }

                    _ => unimplemented!("sub code `{}` not supported for opcode {}", sub, op),
                }
            }

            // I-Type instructions:
            OPCODE_ADDI | OPCODE_ADDIU | OPCODE_DADDI | OPCODE_DADDIU | OPCODE_LW | OPCODE_SW
            | OPCODE_LUI | OPCODE_ORI | OPCODE_ANDI | OPCODE_REGIMM | OPCODE_BEQ | OPCODE_BNE => {
                Instruction::IType(IType {
                    op: ((value >> 26) & 0x3F) as u8,
                    rs: ((value >> 21) & 0x1F) as u8,
                    rt: ((value >> 16) & 0x1F) as u8,
                    immediate: (value & 0xFFFF) as u16,
                })
            }

            // Store/load word to Coprocessor 1
            OPCODE_SWC1 | OPCODE_LWC1 => Instruction::FpuIType(FpuIType {
                op: ((value >> 26) & 0x3F) as u8,
                base: ((value >> 21) & 0x1F) as u8,
                ft: ((value >> 16) & 0x1F) as u8,
                offset: (value & 0xFFFF) as u16,
            }),

            OPCODE_J | OPCODE_JAL => Instruction::JType(JType {
                op: ((value >> 26) & 0x3F) as u8,
                addr: value & 0x03ffffff,
            }),
            _ => unimplemented!("opcode `{}` not supported", op),
        }
    }
}
