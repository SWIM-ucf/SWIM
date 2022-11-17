#[derive(Default)]
pub struct Registers {
    pub pc: u64,
    pub gpr: [u64; 32],
    pub fpr: [u64; 32],
    pub cc: u64,
}
