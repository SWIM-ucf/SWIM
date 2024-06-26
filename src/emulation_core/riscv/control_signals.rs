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

/// Selection of different Immediate forms.
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

/// Selection of different sources for the first operand.
#[derive(Clone, Default, PartialEq)]
pub enum OP1Select {
    #[default]
    DATA1,
    PC,
    IMM,
}

/// Selection of different sources for the second operand.
#[derive(Clone, Default, PartialEq)]
pub enum OP2Select {
    DATA2,
    #[default]
    IMM,
}

/// The output of the ALU control unit that directly controls the ALU.
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

    MultiplicationSignedUpper,

    MultiplicationSignedUnsignedUpper,

    MultiplicationUnsignedSignedUpper,

    RemainderSigned,

    RemainderUnsigned,
}

/// Selection of System Operations.
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

/// Selection of Branch Operations.
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

/// Selection of Read and Write Operations.
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

/// Determines, given [`RegWriteEn`] is set, what the source of a
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

/// Determines, given that a write value in [`ReadWrite`] is set, the source of the data
/// will be written to memory.
#[derive(Clone, Default, PartialEq)]
pub enum MemWriteSrc {
    /// Source the write data from the main processing unit. Specifically, this means the data read from the register `rs1` from a given instruction.
    #[default]
    PrimaryUnit = 0,

    /// Source the write data from the floating-point unit.
    FloatingPointUnit = 1,
}

/// Determines, given that [`RegWriteEn`] is set, which destination
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

/// Floating Point Control Signals Module.
pub mod floating_point {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub struct FpuControlSignals {
        pub round_mode: RoundingMode,
        pub data_src: DataSrc,
        pub data_write: DataWrite,
        pub fpu_alu_op: FpuAluOp,
        pub fpu_mem_to_reg: FpuMemToReg,
        pub fpu_reg_dst: FpuRegDst,
        pub fpu_reg_write: FpuRegWrite,
    }

    /// Determines the source of the `Data` register in the floating-point unit.
    ///
    /// This is a special intermediary register that facilitates passing data between
    /// the main processing unit and the floating-point unit.
    #[derive(Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
    pub enum DataSrc {
        /// Use data from the main processing unit. Specifically, the data from register
        /// `rs1` from a given instruction. This value can additionally be used in the cases
        /// where this register is not written to.
        MainProcessorUnit = 0,

        /// Use data from the floating-point unit. Specifically, the data from register `rs1`
        /// from a given instruction.
        #[default]
        FloatingPointUnitRS1 = 1,

        /// Use data from the floating-point unit. Specifically, the data from the comparator.
        FloatingPointUnitComp = 2,

        /// Use data from the floating-point unit. Specifically, the Classify Mask.
        FloatingPointUnitMask = 3,

        /// Use the un-altered bits from the floating-point unit.
        FloatingPointBits = 4,

        /// Use the un-altered bits from the main unit.
        MainProcessorBits = 5,
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
        ///   in the FPU, likely from register `rs1` from a given instruction. This data source
        ///   overrides the decision given by the [`WBSel`](super::WBSel) control signal.
        /// - Source data to write to the floating-point register file from the `Data` register
        ///   in the FPU, likely from register `rs1` from a given instruction.
        YesWrite = 1,
    }

    /// This doubly determines the operations sent to the floating-point ALU and the
    /// floating-point comparator.
    ///
    /// Only one of these units are effectively utilized in any given instruction.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub enum FpuAluOp {
        #[default]
        /// `_00000` (0):
        /// - ALU: Perform an addition.
        Addition = 0,

        /// `_00001` (1):
        /// - ALU: Perform a subtraction.
        Subtraction = 1,

        /// `_00010` (2):
        /// - ALU: Perform a multiplication.
        /// - Comparator: Set if equal.
        MultiplicationOrEqual = 2,

        /// `_00011` (3):
        /// - ALU: Perform a division.
        Division = 3,

        /// `_00100` (4):
        /// - ALU: Perform a Square Root.
        Sqrt = 4,

        /// `_00101` (5):
        /// - ALU: Take the Minimum value.
        Min = 5,

        /// `_00110` (6):
        /// - ALU: Take the Maximum value.
        Max = 6,

        /// `_00111` (7):
        /// - ALU: Sign-Injection.
        SGNJ = 7,

        /// `_01000` (8):
        /// - ALU: Negative Sign-Injection.
        SGNJN = 8,

        /// `_01001` (9):
        /// - ALU: Xor Sign-Injection.
        SGNJX = 9,

        /// `_01010` (10):
        /// - ALU: Classification Mask.
        Class = 10,

        /// `_01011` (11):
        /// - ALU: Fused Multiplication-Addition.
        MAdd = 11,

        /// `_01100` (12):
        /// - ALU: Fused Multiplication-Subtraction.
        MSub = 12,

        /// `_01101` (13):
        /// - ALU: Fused Negated Multiplication-Subtraction.
        NMSub = 13,

        /// `_01110` (14):
        /// - ALU: Fused Negated Multiplication-Addition.
        NMAdd = 14,

        /// `_10000` (16):
        /// - Comparator: Set if less than.
        Slt = 16,

        /// `_10001` (17):
        /// - Comparator: Set if less than or equal.
        Sle = 17,
    }

    /// Selection of Rounding Modes
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
}
