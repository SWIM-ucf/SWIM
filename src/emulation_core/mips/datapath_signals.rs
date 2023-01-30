//! Internal datapath signals.

// The random mid-datapath-signals
#[derive(Default, PartialEq)]
pub struct DatapathSignals {
    pub alu_z: AluZ,
}

/// The Z like comming off the main ALU
#[derive(Default, PartialEq)]
pub enum AluZ {
    /// alut_result is not zero
    #[default]
    NotZero = 0,

    /// alu_result is zero
    YesZero = 1,
}
