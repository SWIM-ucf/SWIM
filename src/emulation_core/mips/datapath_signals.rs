//! Internal datapath signals.

// The random mid-datapath-signals
#[derive(Clone, Default, PartialEq)]
pub struct DatapathSignals {
    pub alu_z: AluZ,
    pub cpu_branch: CpuBranch,
    // pub fpu_branch: FpuBranch,
    pub general_branch: GeneralBranch,
}

/// The Z like comming off the main ALU
#[derive(Clone, Default, PartialEq)]
pub enum AluZ {
    /// alut_result is not zero
    #[default]
    NotZero = 0,

    /// alu_result is zero
    YesZero = 1,
}

/// CPU branch signal
///
/// This signal is set in the EX stage
#[derive(Clone, Default, PartialEq)]
pub enum CpuBranch {
    /// (branch control signal != YesBranch) || (AluZ != YesZero)
    #[default]
    NoBranch = 0,

    /// (branch control signal == YesBranch) && (AluZ == YesZero)
    YesBranch = 1,
}

// For now this signal sits in constrol_signals.rs
//pub enum FpuBranch {
//     // FPU has not signaled for a branch
//     #[default]
//     NoBranch = 0,
//
//     // FPU wants a branch
//     YesBranch = 1,
//}

/// General branch signal
///
/// This signal is set in the MEM stage.
/// GeneralBranch = CpuBranch | FpuBranch
#[derive(Clone, Default, PartialEq)]
pub enum GeneralBranch {
    #[default]
    NoBranch = 0,
    YesBranch = 1,
}
