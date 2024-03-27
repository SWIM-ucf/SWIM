//! Internal datapath control signals.

/// Full collection of control signals.
#[derive(Clone, Default, PartialEq)]
pub struct ControlSignals {
    pub imm_select: ImmSelect,
    pub op1_select: OP1Select,
    pub op2_select: OP2Select,
    pub alu_op: AluOp,
    pub sys_op: SysOp,
    pub branch_jump: BranchJump,
    pub read_write: ReadWrite,
    pub wb_sel: WBSel,
    pub mem_write_src: MemWriteSrc,
    pub reg_dst: RegDst,
    pub reg_write_en: RegWriteEn,
}

#[derive(Clone, Default, PartialEq)]
pub enum ImmSelect {
    UType,
    JType,
    SType,
    BType,
    #[default]
    ISigned,
    IShamt,
    IUnsigned,
}

#[derive(Clone, Default, PartialEq)]
pub enum OP1Select {
    #[default]
    DATA1,
    PC,
    IMM,
}

#[derive(Clone, Default, PartialEq)]
pub enum OP2Select {
    DATA2,
    #[default]
    IMM,
}

/// The output of the ALU control unit that directly controls the ALU.
///
/// This is not to be confused with the [`AluOp`] signal. ALUControl is
/// a function of both [`AluOp`] and the `funct` field of an R-type
/// instruction. While this is a separate control signal, the
/// [`RegWidth`] control signal additionally acts as the leading bit of
/// ALUControl. The leading bit of the signal determines the size of
/// the input and output data within the datapath. See [`RegWidth`] for
/// more details.
#[derive(Clone, Default, PartialEq)]
pub enum AluOp {
    /// `_0000` (0) - Perform an addition. (Also used in cases where the ALU result does not matter.)
    #[default]
    Addition,

    /// `_0001` (1) - Perform a subtraction. Will not set any underflow signal on underflow.
    Subtraction,

    /// `_0010` (2) - Perform a shift left logical operation by `shamt` bits.
    ShiftLeftLogical(u32),

    /// `_0011` (3) - Perform a "set on less than" operation.
    SetOnLessThanSigned,

    /// `_0100` (4) - Perform a "set on less than unsigned" operation.
    SetOnLessThanUnsigned,

    /// `_0101` (5) - Perform a bitwise "Xor" operation.
    Xor,

    /// `_0110` (6) - Perform a shift right logical operation by `shamt` bits.
    ShiftRightLogical(u32),

    /// `_0111` (7) - Perform a shift right arithmetic operation by `shamt` bits.
    ShiftRightArithmetic(u32),

    /// `_1000` (8) - Perform a bitwise "OR" operation.
    Or,

    /// `_1001` (9) - Perform a bitwise "AND" operation.
    And,

    /// `_1010` (10) - Left shift the sign-extended immediate value 16 bits.
    LeftShift16,

    /// `_1011` (11) - Perform signed multiplication.
    MultiplicationSigned,

    /// `_1100` (12) - Perform unsigned multiplication.
    MultiplicationUnsigned,

    /// `_1101` (13) - Perform signed integer division. (Returns the integer quotient.)
    DivisionSigned,

    /// `_1110` (14) - Perform unsigned integer division. (Returns the integer quotient.)
    DivisionUnsigned,
}

#[derive(Clone, Default, PartialEq)]
pub enum SysOp {
    #[default]
    None,
    ECALL,
    EBREAK,
    CSRReadWrite,
    CSRReadSet,
    CSRReadClear,
}

#[derive(Clone, Default, PartialEq)]
pub enum BranchJump {
    Beq,
    Bne,
    #[default]
    NoBranch,
    J,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Clone, Default, PartialEq)]
pub enum ReadWrite {
    #[default]
    NoLoadStore,
    LoadByte,
    LoadHalf,
    LoadWord,
    LoadDouble,
    LoadByteUnsigned,
    LoadHalfUnsigned,
    LoadWordUnsigned,
    StoreByte,
    StoreHalf,
    StoreWord,
    StoreDouble,
}

/// Determines, given [`RegWrite`] is set, what the source of a
/// register's new data will be.
///
/// The decision can be completely overridden by the floating point
/// unit's [`DataWrite`](floating_point::DataWrite) control signal.
///
/// This control signal also applies to what data is sent to the
/// floating-point unit to be stored in its registers.
#[derive(Clone, Default, PartialEq)]
pub enum WBSel {
    #[default]
    UseAlu = 0,
    UseMemory = 1,
    UseImmediate = 2,
    UsePcPlusFour = 3,
}

/// Determines, given that [`MemWrite`] is set, the source of the data
/// will be written to memory.
///
/// Compared to the general-purpose datapath introduced by Hennessy and
/// Patterson, this is a new control signal created to incorporate the
/// floating-point unit.
#[derive(Clone, Default, PartialEq)]
pub enum MemWriteSrc {
    /// Source the write data from the main processing unit. Specifically, this means the data read from the register `rt` from a given instruction.
    #[default]
    PrimaryUnit = 0,

    /// Source the write data from the floating-point unit. Specifically, this means the data read from the register `ft` from a given instruction.
    FloatingPointUnit = 1,
}

/// Determines, given that [`RegWrite`] is set, which destination
/// register to write to, which largely depends on the instruction format.
#[derive(Clone, Default, PartialEq)]
pub enum RegDst {
    /// Use register `rs1`.
    Reg1 = 0,

    /// Use register `rs2`.
    Reg2 = 1,

    /// Use register `rd`.
    #[default]
    Reg3 = 2,

    /// Write to general-purpose register 31 ($ra). This is the return address
    /// used in `jal` instructions.
    ReturnRegister = 3,
}

/// Determines if the register file should be written to.
#[derive(Clone, Default, Eq, PartialEq)]
pub enum RegWriteEn {
    #[default]
    NoWrite = 0,
    YesWrite = 1,
}
