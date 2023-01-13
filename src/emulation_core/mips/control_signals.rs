//! Internal datapath control signals.

/// Full collection of control signals.
#[derive(Default)]
pub struct ControlSignals {
    pub alu_control: AluControl,
    pub alu_op: AluOp,
    pub alu_src: AluSrc,
    pub branch: Branch,
    pub imm_shift: ImmShift,
    pub jump: Jump,
    pub mem_read: MemRead,
    pub mem_to_reg: MemToReg,
    pub mem_write: MemWrite,
    pub mem_write_src: MemWriteSrc,
    pub reg_dst: RegDst,
    pub reg_width: RegWidth,
    pub reg_write: RegWrite,
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
#[derive(Default)]
pub enum AluControl {
    /// `_0000` (0) - Perform an addition. (Also used in cases where the ALU result does not matter.)
    #[default]
    Addition = 0,

    /// `_0001` (1) - Perform a subtraction.
    Subtraction = 1,

    /// `_0010` (2) - Perform a "set on less than" operation.
    SetOnLessThanSigned = 2,

    /// `_0011` (3) - Perform a "set on less than unsigned" operation.
    SetOnLessThanUnsigned = 3,

    /// `_0100` (4) - Perform a bitwise "AND" operation.
    And = 4,

    /// `_0101` (5) - Perform a bitwise "OR" operation.
    Or = 5,

    /// `_0110` (6) - Left shift the sign-extended immediate value 16 bits.
    LeftShift16 = 6,

    /// `_0111` (7) - Perform a bitwise "NOT" operation.
    Not = 7,

    /// `_1000` (8) - Perform signed multiplication.
    MultiplicationSigned = 8,

    /// `_1001` (9) - Perform unsigned multiplication.
    MultiplicationUnsigned = 9,

    /// `_1010` (10) - Perform signed integer division. (Returns the integer quotient.)
    DivisionSigned = 10,

    /// `_1011` (11) - Perform unsigned integer division. (Returns the integer quotient.)
    DivisionUnsigned = 11,
}

/// This determines the operation sent to the ALU control unit.
///
/// This is on a higher abstraction than the output of this control
/// unit, which more specifically determines what operation the ALU
/// will perform.
#[derive(Default)]
pub enum AluOp {
    /// `000` (0) - Perform an addition. (Also used in cases where the ALU result does not matter.)
    #[default]
    Addition = 0,

    /// `001` (1) - Perform a subtraction.
    Subtraction = 1,

    /// `010` (2) - Perform a "set on less than" operation.
    SetOnLessThanSigned = 2,

    /// `011` (3) - Perform a "set on less than unsigned" operation.
    SetOnLessThanUnsigned = 3,

    /// `100` (4) - Perform a binary "AND" operation.
    And = 4,

    /// `101` (5) - Perform a binary "OR" operation.
    Or = 5,

    /// `110` (6) - Left shift the sign-extended immediate value 16 bits.
    LeftShift16 = 6,

    /// `111` (7) - This is an R-type instruction and the operation
    /// should instead refer to the `funct` field in the instruction.
    ///
    /// (Note: For the `mul` and `div` instructions, the operation of
    /// the ALU may additionally be determined by bits 10-6 of the
    /// instruction (same bits as the `shamt` field), as the `funct`
    /// field alone does not provide the full description of those
    /// instructions.)
    UseFunctField = 7,
}

/// Determines the second source of the ALU.
///
/// The first input is always the data read from the register `rs` (or
/// called `base` in some contexts.)
#[derive(Default)]
pub enum AluSrc {
    /// Use the data from the from the second source register `rt`.
    #[default]
    ReadRegister2 = 0,

    /// Use the sign-extended 16-bit immediate field in the instruction. This may be left-shifted by some amount given by the [`ImmShift`] control signal.
    SignExtendedImmediate = 1,

    /// Use the zero-extended 16-bit immediate field in the instruction.
    ZeroExtendedImmediate = 2,
}

/// Determines if the datapath should consider branching.
///
/// Exact choice of branching or not branching relies on the result from the ALU.
#[derive(Default)]
pub enum Branch {
    /// Do not consider branching.
    #[default]
    NoBranch = 0,

    /// Consider branching.
    YesBranch = 1,
}

/// Determines the amount of bits to left-shift the immediate value before being passed to the ALU.
#[derive(Default)]
pub enum ImmShift {
    #[default]
    Shift0 = 0,
    Shift16 = 1,
    Shift32 = 2,
    Shift48 = 3,
}

/// Determines if the datapath should jump. This is an unconditional branch.
#[derive(Default)]
pub enum Jump {
    #[default]
    NoJump = 0,
    YesJump = 1,
}

/// Determines if memory should be read.
///
/// This should not be set in combination with [`MemWrite`].
#[derive(Default)]
pub enum MemRead {
    #[default]
    NoRead = 0,
    YesRead = 1,
}

/// Determines, given [`RegWrite`] is set, what the source of a
/// register's new data will be.
///
/// The decision can be completely overridden by the floating point
/// unit's [`DataWrite`] control signal.
#[derive(Default)]
pub enum MemToReg {
    #[default]
    UseAlu = 0,
    UseMemory = 1,
}

/// Determines if memory should be written to.
///
/// This should not be set in combination with the [`MemRead`] control signal.
#[derive(Default)]
pub enum MemWrite {
    #[default]
    NoWrite = 0,
    YesWrite = 1,
}

/// Determines, given that [`MemWrite`] is set, the source of the data
/// will be written to memory.
///
/// Compared to the general-purpose datapath introduced by Hennessy and
/// Patterson, this is a new control signal created to incorporate the
/// floating-point unit.
#[derive(Default)]
pub enum MemWriteSrc {
    /// Source the write data from the main processing unit. Specifically, this means the data read from the register `rt` from a given instruction.
    #[default]
    PrimaryUnit = 0,

    /// Source the write data from the floating-point unit. Specifically, this means the data read from the register `ft` from a given instruction.
    FloatingPointUnit = 1,
}

/// Determines, given that [`RegWrite`] is set, which destination
/// register to write to, which largely depends on the instruction format.
#[derive(Default)]
pub enum RegDst {
    /// Use register `rs`.
    Reg1 = 0,

    /// Use register `rt`.
    Reg2 = 1,

    /// Use register `rd`.
    #[default]
    Reg3 = 2,
}

/// Determines the amount of data to be sent or recieved from registers
/// and the ALU. While all buses carrying information are 64 bits wide,
/// some bits of the bus may be ignored in the case of this control
/// signal.
#[derive(Default)]
pub enum RegWidth {
    /// Use words (32 bits).
    Word = 0,

    /// Use doublewords (64 bits).
    #[default]
    DoubleWord = 1,
}

/// Determines if the register file should be written to.
#[derive(Default, Eq, PartialEq)]
pub enum RegWrite {
    #[default]
    NoWrite = 0,
    YesWrite = 1,
}
