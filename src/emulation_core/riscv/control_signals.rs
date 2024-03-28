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

pub mod floating_point {

    use super::super::constants::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub struct FpuControlSignals {
        pub round_mode: RoundingMode,
        pub data_src: DataSrc,
        pub data_write: DataWrite,
        pub fpu_alu_op: FpuAluOp,
        pub fpu_branch: FpuBranch,
        pub fpu_mem_to_reg: FpuMemToReg,
        pub fpu_reg_dst: FpuRegDst,
        pub fpu_reg_write: FpuRegWrite,
        pub fpu_take_branch: FpuTakeBranch,
    }

    /// Determines the source of the `Data` register in the floating-point unit.
    ///
    /// This is a special intermediary register that facilitates passing data between
    /// the main processing unit and the floating-point unit.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum DataSrc {
        /// Use data from the main processing unit. Specifically, the data from register
        /// `rt` from a given instruction. This value can additionally be used in the cases
        /// where this register is not written to.
        MainProcessorUnit = 0,

        /// Use data from the floating-point unit. Specifically, the data from register `fs`
        /// from a given instruction.
        #[default]
        FloatingPointUnit = 1,
    }

    /// Determines whether to write to the `Data` register in the floating-point unit.
    ///
    /// This acts as a toggle for the source of data to the main processing unit register
    /// file. Additionally, it acts as a toggle for a source to the floating-point unit
    /// register file (this could be overridden by the [`FpuMemToReg`] control signal).
    /// For the latter two functions, it is imperative to unset the [`RegWriteEn`](super::RegWriteEn) and
    /// [`FpuRegWrite`] control signals in cases where registers should not be modified
    /// with unintended data.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum DataWrite {
        /// - Do not write to the data register.
        /// - Source data to write to the main processing unit register file from the main
        ///   processing unit. This implies either the ALU result or the data read from memory
        /// - Source data to write to the floating-point register file from the floating-point
        ///   ALU.
        #[default]
        NoWrite = 0,

        /// - Write to the data register.
        /// - Source data to write to the main processing unit register file from the
        ///   floating-point unit. Specifically, this is the data stored in the `Data` register
        ///   in the FPU, likely from register `fs` from a given instruction. This data source
        ///   overrides the decision given by the [`MemToReg`](super::MemToReg) control signal.
        /// - Source data to write to the floating-point register file from the `Data` register
        ///   in the FPU, likely from register `rt` from a given instruction.
        YesWrite = 1,
    }

    /// This doubly determines the operations sent to the floating-point ALU and the
    /// floating-point comparator.
    ///
    /// Only one of these units are effectively utilized in any given instruction.
    ///
    /// The fifth bit of the control signal represents either a single-precision
    /// floating-point operation (0), or a double-precision floating-point operation (1).
    /// This fifth bit is determined by [`FpuRegWidth`].
    ///
    /// *Implementation note:* The bits set for the comparator are intended to match
    /// the bits used in the `cond` field of a `c.cond.fmt` instruction.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub enum FpuAluOp {
        #[default]
        /// `_0000` (0):
        /// - ALU: Perform an addition.
        Addition = 0,

        /// `_0001` (1):
        /// - ALU: Perform a subtraction.
        Subtraction = 1,

        /// `_0010` (2):
        /// - ALU: Perform a multiplication.
        /// - Comparator: Set if equal.
        MultiplicationOrEqual = 2,

        /// `_0011` (3):
        /// - ALU: Perform a division.
        Division = 3,

        /// `_0100` (4):
        /// - ALU: Perform a Square Root.
        Sqrt = 4,

        /// `_0101` (5):
        /// - ALU: Perform an "OR" operation.
        Or = 5,

        /// `_1100` (12):
        /// - Comparator: Set if less than.
        Slt = 12,

        /// `_1101` (13):
        /// - Comparator: Set if not greater than or equal.
        Snge = 13,

        /// `_1110` (14):
        /// - Comparator: Set if less than or equal.
        Sle = 14,

        /// `_1111` (15):
        /// - Comparator: Set if not greater than.
        Sngt = 15,
    }

    impl FpuAluOp {
        /// Get the corresponding control signal given a function code.
        pub fn from_function(function: u8) -> Result<Self, String> {
            match function {
                FUNCTION_C_EQ => Ok(Self::MultiplicationOrEqual),
                FUNCTION_C_LT => Ok(Self::Slt),
                FUNCTION_C_NGE => Ok(Self::Snge),
                FUNCTION_C_LE => Ok(Self::Sle),
                FUNCTION_C_NGT => Ok(Self::Sngt),
                _ => Err(format!("Unsupported function code `{function}`")),
            }
        }
    }
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub enum RoundingMode {
        RNE = 0,
        RTZ = 1,
        RDN = 2,
        RUP = 3,
        RMM = 4,
        #[default]
        DRM = 7,
    }

    /// Determines if the floating-point unit should consider branching, based on the
    /// contents of the condition code register.
    ///
    /// This directly overrides any branch decisions decided by the main processing unit.
    /// The [`Branch`](super::Branch) control signal should not be set in addition to this signal.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum FpuBranch {
        /// Do not consider branching.
        #[default]
        NoBranch = 0,

        /// Consider branching.
        YesBranch = 1,
    }

    /// Determines, given that [`FpuRegWrite`] is set, what the source of a floating-point
    /// register's new data will be.
    ///
    /// This decision, if set, overrides the decision from the [`DataWrite`] control signal.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum FpuMemToReg {
        /// Do not use data from memory. Use the result of the [`DataWrite`] control signal.
        #[default]
        UseDataWrite = 0,

        /// Use data from memory.
        UseMemory = 1,
    }

    /// Determines, given that [`FpuRegWrite`] is set, which destination register to write
    /// to, which largely depends on the instruction format.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum FpuRegDst {
        /// Use register `rs1`.
        Reg1 = 0,

        /// Use register `rs2`.
        Reg2 = 1,

        /// Use register `rd`.
        #[default]
        Reg3 = 2,
    }

    /// Determines if the floating-point register file should be written to.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum FpuRegWrite {
        /// Do not write to the floating-point register file.
        #[default]
        NoWrite = 0,

        /// Write to the floating-point register file.
        YesWrite = 1,
    }

    /// After checking the [`FpuBranch`] and condition code, this signal determines whether
    /// to follow through with a branch.
    ///
    /// This signal is what is sent to the main processor.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum FpuTakeBranch {
        #[default]
        NoBranch = 0,
        YesBranch = 1,
    }
}
