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
            _ => LineInformation {
                title: String::from("[Title]"),
                description: String::from("[Description]"),
                value: 0,
                bits: 0,
            },
        }
    }
}
