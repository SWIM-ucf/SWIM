#[derive(Default)]
pub struct ControlSignals {
    pub alu_control: AluControl,
    pub alu_op: AluOp,
    pub alu_src: AluSrc,
    pub branch: Branch,
    pub jump: Jump,
    pub mem_read: MemRead,
    pub mem_to_reg: MemToReg,
    pub mem_write: MemWrite,
    pub mem_write_src: MemWriteSrc,
    pub reg_dst: RegDst,
    pub reg_write: RegWrite,
}

#[derive(Default)]
pub enum AluControl {
    #[default]
    Addition = 0,
    Subtraction = 1,
    SetOnLessThanSigned = 2,
    SetOnLessThanUnsigned = 3,
    And = 4,
    Or = 5,
    LeftShift16 = 6,
    Not = 7,
}

#[derive(Default)]
pub enum AluOp {
    #[default]
    Addition = 0,
    Subtraction = 1,
    SetOnLessThanSigned = 2,
    SetOnLessThanUnsigned = 3,
    And = 4,
    Or = 5,
    LeftShift16 = 6,
    UseFunctField = 7,
}

#[derive(Default)]
pub enum AluSrc {
    #[default]
    ReadRegister2 = 0,
    ExtendedImmediate = 1,
}

#[derive(Default)]
pub enum Branch {
    #[default]
    NoBranch = 0,
    YesBranch = 1,
}

#[derive(Default)]
pub enum Jump {
    #[default]
    NoJump = 0,
    YesJump = 1,
}

#[derive(Default)]
pub enum MemRead {
    #[default]
    NoRead = 0,
    YesRead = 1,
}

#[derive(Default)]
pub enum MemToReg {
    #[default]
    UseAlu = 0,
    UseMemory = 1,
}

#[derive(Default)]
pub enum MemWrite {
    #[default]
    NoWrite = 0,
    YesWrite = 1,
}

#[derive(Default)]
pub enum MemWriteSrc {
    #[default]
    PrimaryUnit = 0,
    FloatingPointUnit = 1,
}

#[derive(Default)]
pub enum RegDst {
    Reg2 = 0,
    #[default]
    Reg3 = 1,
}

#[derive(Default, Eq, PartialEq)]
pub enum RegWrite {
    #[default]
    NoWrite = 0,
    YesWrite = 1,
}
