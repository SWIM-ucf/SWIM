//! Abstract representation of an instruction.

#[derive(Debug, Default, Clone, Copy)]
pub struct RType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub shamt: u8,
    pub funct: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct IType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub immediate: u16,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct JType {
    pub op: u8,
    pub addr: u32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FpuRType {
    pub op: u8,
    pub fmt: u8,
    pub ft: u8,
    pub fs: u8,
    pub fd: u8,
    pub function: u8,
}

#[derive(Debug)]
pub enum Instruction {
    RType(RType),
    IType(IType),
    JType(JType),
    FpuRType(FpuRType),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::RType(RType::default())
    }
}

impl From<u32> for Instruction {
    /// Based on the opcode, convert a binary instruction into a struct representation.
    fn from(value: u32) -> Self {
        let op: u32 = value >> 26;
        match op {
            // R-type instructions (add, sub, mul, div, and, or, slt, sltu)
            0 => Instruction::RType(RType {
                op: ((value >> 26) & 0x3F) as u8,
                rs: ((value >> 21) & 0x1F) as u8,
                rt: ((value >> 16) & 0x1F) as u8,
                rd: ((value >> 11) & 0x1F) as u8,
                shamt: ((value >> 6) & 0x1F) as u8,
                funct: (value & 0x3F) as u8,
            }),

            // COP1 (coprocessor 1)
            0b010001 => Instruction::FpuRType(FpuRType {
                op: ((value >> 26) & 0x3F) as u8,
                fmt: ((value >> 21) & 0x1F) as u8,
                ft: ((value >> 16) & 0x1F) as u8,
                fs: ((value >> 11) & 0x1F) as u8,
                fd: ((value >> 6) & 0x1F) as u8,
                function: (value & 0x3F) as u8,
            }),

            // Load Word (lw)
            0b100011 => Instruction::IType(IType {
                op: ((value >> 26) & 0x3F) as u8,
                rs: ((value >> 21) & 0x1F) as u8,
                rt: ((value >> 16) & 0x1F) as u8,
                immediate: (value & 0xFFFF) as u16,
            }),

            // Or immediate (ori)
            0b001101 => Instruction::IType(IType {
                op: ((value >> 26) & 0x3F) as u8,
                rs: ((value >> 21) & 0x1F) as u8,
                rt: ((value >> 16) & 0x1F) as u8,
                immediate: (value & 0xFFFF) as u16,
            }),
            _ => todo!("Unsupported opcode"),
        }
    }
}
