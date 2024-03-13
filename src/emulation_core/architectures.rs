use crate::emulation_core::mips::datapath::MipsDatapath;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AvailableDatapaths {
    MIPS,
    RISCV,
}

pub enum DatapathRef<'a> {
    MIPS(&'a MipsDatapath),
}
