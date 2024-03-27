use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::riscv::datapath::RiscDatapath;
use core::fmt;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum AvailableDatapaths {
    MIPS,
    RISCV,
}

impl fmt::Display for AvailableDatapaths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AvailableDatapaths::MIPS => write!(f, "MIPS"),
            AvailableDatapaths::RISCV => write!(f, "RISCV"),
        }
    }
}

// from string
impl From<&str> for AvailableDatapaths {
    fn from(s: &str) -> Self {
        match s {
            "MIPS" => AvailableDatapaths::MIPS,
            "RISCV" => AvailableDatapaths::RISCV,
            _ => panic!("Invalid datapath type"),
        }
    }
}

pub enum DatapathRef<'a> {
    MIPS(&'a MipsDatapath),
    RISCV(&'a RiscDatapath),
}
