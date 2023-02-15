//! Module for mapping lines in the visual datapath to information
//! and variables in the coded datapath.

use super::super::datapath::VisualDatapath;
use super::datapath::MipsDatapath;

/// A collection of data surrounding a line in the visual datapath.
pub struct LineInformation {
    pub title: String,
    pub description: String,

    /// The value stored in a line. This may not be a 64-bit value, but should
    /// refer to the `bits` field to determine how many bits on the line are
    /// relevant to be displayed.
    pub value: u64,

    /// The number of bits on a given line.
    pub bits: u64,
}

impl VisualDatapath for MipsDatapath {
    type LineInformation = LineInformation;

    fn visual_line_to_data(&self, variable: &str) -> LineInformation {
        match variable {
            "pc" => LineInformation {
                title: String::from("Program Counter"),
                description: String::from("The address of the instruction to be executed next."),
                value: self.registers.pc,
                bits: 32,
            },

            "register_write_data" => LineInformation {
                title: String::from("Register Write Data"),
                description: String::from("Data that will be written to a general-purpose register."),
                value: self.state.register_write_data,
                bits: 64,
            },

            "rt" => LineInformation {
                title: String::from("Instruction [20-16]"),
                description: String::from("The rt field. Contains the second register to be read for an R-type instruction."),
                value: self.state.rt as u64,
                bits: 5,
            },

            "coprocessor.fs" => LineInformation {
                title: String::from("Instruction [15-11]"),
                description: String::from("The fs field. Contains the first register to be read in a floating-point instruction."),
                value: self.coprocessor.state.fs as u64,
                bits: 5,
            },

            _ => LineInformation {
                title: String::from("[Title]"),
                description: String::from("[Description]"),
                value: 0,
                bits: 0,
            },
        }
    }
}
