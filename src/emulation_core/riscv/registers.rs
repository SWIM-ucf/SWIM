//! Register structure and API.

use crate::emulation_core::mips::memory::CAPACITY_BYTES;
use crate::emulation_core::register::{RegisterType, Registers};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

/// Collection of general-purpose registers used by the datapath.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RiscGpRegisters {
    pub pc: u64,
    pub gpr: [u64; 32],
}

/// Specifies all of the valid registers accessible in an instance
/// of [`RiscGpRegisters`].
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
pub enum RiscGpRegisterType {
    Pc = -1,
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    X30 = 30,
    X31 = 31,
}

impl RegisterType for RiscGpRegisterType {
    fn get_register_name(&self) -> String {
        match self {
            RiscGpRegisterType::Pc => self.to_string(),
            _ => format!("{}", self),
        }
    }
    fn is_valid_register_value(&self, value: u64, pc_limit: usize) -> bool {
        match self {
            RiscGpRegisterType::X0 => false, // Zero register is immutable
            RiscGpRegisterType::Pc => {
                // Check if PC is more than the number of instructions or not word-aligned
                value <= pc_limit as u64 && value % 4 == 0
            }
            RiscGpRegisterType::X2 => {
                // Check if SP is more than memory capacity or not word-aligned
                value <= CAPACITY_BYTES as u64 && value % 4 == 0
            }
            _ => true, // Other registers are always considered valid
        }
    }
}

impl Registers for RiscGpRegisters {
    fn get_dyn_register_list(&self) -> Vec<(Rc<dyn RegisterType>, u64)> {
        self.into_iter()
            .map(|(register, val)| {
                let register: Rc<dyn RegisterType> = Rc::new(register);
                (register, val)
            })
            .collect()
    }
}

impl ToString for RiscGpRegisters {
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("PC = {}\n", self.pc));

        let gpr_registers = self
            .gpr
            .iter()
            .enumerate()
            .map(|(i, inst)| format!("gpr[{i}] = {inst}"))
            .collect::<Vec<String>>()
            .join("\n");
        output.push_str(&gpr_registers);

        output
    }
}

impl Index<&str> for RiscGpRegisters {
    type Output = u64;

    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index(&self, index: &str) -> &Self::Output {
        match RiscGpRegisterType::from_str(index) {
            Ok(register) => &self[register],
            _ => panic!("{index} is not a valid register"),
        }
    }
}

impl IndexMut<&str> for RiscGpRegisters {
    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match RiscGpRegisterType::from_str(index) {
            Ok(register) => &mut self[register],
            _ => panic!("{index} is not a valid register"),
        }
    }
}

impl Index<RiscGpRegisterType> for RiscGpRegisters {
    type Output = u64;

    fn index(&self, index: RiscGpRegisterType) -> &Self::Output {
        match index {
            RiscGpRegisterType::Pc => &self.pc,
            RiscGpRegisterType::X0 => &self.gpr[0],
            RiscGpRegisterType::X1 => &self.gpr[1],
            RiscGpRegisterType::X2 => &self.gpr[2],
            RiscGpRegisterType::X3 => &self.gpr[3],
            RiscGpRegisterType::X4 => &self.gpr[4],
            RiscGpRegisterType::X5 => &self.gpr[5],
            RiscGpRegisterType::X6 => &self.gpr[6],
            RiscGpRegisterType::X7 => &self.gpr[7],
            RiscGpRegisterType::X8 => &self.gpr[8],
            RiscGpRegisterType::X9 => &self.gpr[9],
            RiscGpRegisterType::X10 => &self.gpr[10],
            RiscGpRegisterType::X11 => &self.gpr[11],
            RiscGpRegisterType::X12 => &self.gpr[12],
            RiscGpRegisterType::X13 => &self.gpr[13],
            RiscGpRegisterType::X14 => &self.gpr[14],
            RiscGpRegisterType::X15 => &self.gpr[15],
            RiscGpRegisterType::X16 => &self.gpr[16],
            RiscGpRegisterType::X17 => &self.gpr[17],
            RiscGpRegisterType::X18 => &self.gpr[18],
            RiscGpRegisterType::X19 => &self.gpr[19],
            RiscGpRegisterType::X20 => &self.gpr[20],
            RiscGpRegisterType::X21 => &self.gpr[21],
            RiscGpRegisterType::X22 => &self.gpr[22],
            RiscGpRegisterType::X23 => &self.gpr[23],
            RiscGpRegisterType::X24 => &self.gpr[24],
            RiscGpRegisterType::X25 => &self.gpr[25],
            RiscGpRegisterType::X26 => &self.gpr[26],
            RiscGpRegisterType::X27 => &self.gpr[27],
            RiscGpRegisterType::X28 => &self.gpr[28],
            RiscGpRegisterType::X29 => &self.gpr[29],
            RiscGpRegisterType::X30 => &self.gpr[30],
            RiscGpRegisterType::X31 => &self.gpr[31],
        }
    }
}

impl IndexMut<RiscGpRegisterType> for RiscGpRegisters {
    fn index_mut(&mut self, index: RiscGpRegisterType) -> &mut Self::Output {
        match index {
            RiscGpRegisterType::Pc => &mut self.pc,
            RiscGpRegisterType::X0 => panic!("The $zero register cannot be accessed as mutable"),
            RiscGpRegisterType::X1 => &mut self.gpr[1],
            RiscGpRegisterType::X2 => &mut self.gpr[2],
            RiscGpRegisterType::X3 => &mut self.gpr[3],
            RiscGpRegisterType::X4 => &mut self.gpr[4],
            RiscGpRegisterType::X5 => &mut self.gpr[5],
            RiscGpRegisterType::X6 => &mut self.gpr[6],
            RiscGpRegisterType::X7 => &mut self.gpr[7],
            RiscGpRegisterType::X8 => &mut self.gpr[8],
            RiscGpRegisterType::X9 => &mut self.gpr[9],
            RiscGpRegisterType::X10 => &mut self.gpr[10],
            RiscGpRegisterType::X11 => &mut self.gpr[11],
            RiscGpRegisterType::X12 => &mut self.gpr[12],
            RiscGpRegisterType::X13 => &mut self.gpr[13],
            RiscGpRegisterType::X14 => &mut self.gpr[14],
            RiscGpRegisterType::X15 => &mut self.gpr[15],
            RiscGpRegisterType::X16 => &mut self.gpr[16],
            RiscGpRegisterType::X17 => &mut self.gpr[17],
            RiscGpRegisterType::X18 => &mut self.gpr[18],
            RiscGpRegisterType::X19 => &mut self.gpr[19],
            RiscGpRegisterType::X20 => &mut self.gpr[20],
            RiscGpRegisterType::X21 => &mut self.gpr[21],
            RiscGpRegisterType::X22 => &mut self.gpr[22],
            RiscGpRegisterType::X23 => &mut self.gpr[23],
            RiscGpRegisterType::X24 => &mut self.gpr[24],
            RiscGpRegisterType::X25 => &mut self.gpr[25],
            RiscGpRegisterType::X26 => &mut self.gpr[26],
            RiscGpRegisterType::X27 => &mut self.gpr[27],
            RiscGpRegisterType::X28 => &mut self.gpr[28],
            RiscGpRegisterType::X29 => &mut self.gpr[29],
            RiscGpRegisterType::X30 => &mut self.gpr[30],
            RiscGpRegisterType::X31 => &mut self.gpr[31],
        }
    }
}

/// Iterator that is used to view each register in the register file.
///
/// This contains a copy of all the registers and their values, and a [`GpRegisterTypeIter`],
/// as generated by [`strum::IntoEnumIterator`]. In other iterator implementations,
/// the internal state might be data like a [`RiscGpRegisterType`]. However, since we can't
/// normally just "add 1" to get to the next register, we use an internal iterator
/// that can track the progression of one [`RiscGpRegisterType`] to the next.
pub struct GpRegistersIter {
    registers: RiscGpRegisters,
    register_iter: RiscGpRegisterTypeIter,
}

/// This implementation of the [`Iterator`] trait essentially wraps the existing
/// [`GpRegisterTypeIter`] so that the register type can be paired with register data.
impl Iterator for GpRegistersIter {
    type Item = (RiscGpRegisterType, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.register_iter.next() {
            Some(register_type) => Some((register_type, self.registers[register_type])),
            None => None,
        }
    }
}

/// [`IntoIterator`] is a standard library trait that can convert any type into
/// an [`Iterator`]. In this case, this is an instance of [`GpRegistersIter`] with all the
/// data in the registers and a new [`GpRegisterTypeIter`].
impl IntoIterator for RiscGpRegisters {
    type Item = (RiscGpRegisterType, u64);
    type IntoIter = GpRegistersIter;

    /// Consumes the [`RiscGpRegisters`] struct to create a new [`GpRegistersIter`] that can
    /// be iterated over.
    fn into_iter(self) -> Self::IntoIter {
        GpRegistersIter {
            registers: self,
            register_iter: RiscGpRegisterType::iter(),
        }
    }
}
