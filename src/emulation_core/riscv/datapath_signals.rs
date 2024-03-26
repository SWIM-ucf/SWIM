//! Internal datapath signals.

#[derive(Clone, Default, PartialEq)]
pub struct DatapathSignals {
    pub alu_z: AluZ,
    pub cpu_branch: CpuBranch,
    pub general_branch: GeneralBranch,
    pub reg_width: RegisterWidth,
}

/// The "Zero" line that comes out of the ALU.
///
/// Indicates whether or not the result of the last arithmetic
/// operation was equal to 0.
#[derive(Clone, Default, PartialEq)]
pub enum AluZ {
    /// The result of the ALU is bitwise zero.
    #[default]
    YesZero = 0,

    /// The result of the ALU is non-zero.
    NoZero = 1,
}

/// CPU branch signal. This is the final determined branch signal from the CPU.
///
/// This signal uses as input the [`Branch`](super::control_signals::Branch),
/// [`BranchType`](super::control_signals::BranchType), and [`AluZ`] signals to
/// determine its value. This signal is set in the EX stage.
#[derive(Clone, Default, PartialEq)]
pub enum CpuBranch {
    /// Do not branch.
    ///
    /// Based on the following formula: `(Branch != YesBranch) || (AluZ != YesZero)`
    #[default]
    NoBranch = 0,

    /// Branch.
    ///
    /// Based on the following formula: `(Branch == YesBranch) && (AluZ == YesZero)`
    YesBranch = 1,
}

/// General branch signal. This is the final determined branch signal from
/// the CPU and FPU combined.
///
/// This signal uses as input the [`CpuBranch`] and [`FpuBranch`](super::control_signals::floating_point::FpuBranch) signals.
/// This signal is set in the MEM stage.
///
/// The following formula is considered: [`GeneralBranch`] = [`CpuBranch`] | [`FpuBranch`](super::control_signals::floating_point::FpuBranch)
#[derive(Clone, Default, PartialEq)]
pub enum GeneralBranch {
    #[default]
    NoBranch = 0,
    YesBranch = 1,
}

#[derive(Clone, Default, PartialEq)]
pub enum RegisterWidth {
    #[default]
    DoubleWidth = 0,
    HalfWidth = 1,
}
