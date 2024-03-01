//! Register structure and API.

use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

/// Collection of general-purpose registers used by the datapath.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FpRegisters {
    pub fpr: [u64; 32],
}

/// Specifies all of the valid registers accessible in an instance
/// of [`FpRegisters`].
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
pub enum FpRegisterType {
    F0 = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
    F6 = 6,
    F7 = 7,
    F8 = 8,
    F9 = 9,
    F10 = 10,
    F11 = 11,
    F12 = 12,
    F13 = 13,
    F14 = 14,
    F15 = 15,
    F16 = 16,
    F17 = 17,
    F18 = 18,
    F19 = 19,
    F20 = 20,
    F21 = 21,
    F22 = 22,
    F23 = 23,
    F24 = 24,
    F25 = 25,
    F26 = 26,
    F27 = 27,
    F28 = 28,
    F29 = 29,
    F30 = 30,
    F31 = 31,
}

impl FpRegisterType {
    pub fn get_fpr_name(&self) -> String {
        format!("f{}", *self as u32)
    }
    pub fn is_valid_register_value(&self, _value: u64) -> bool {
        true
    }
}

impl ToString for FpRegisters {
    fn to_string(&self) -> String {
        let mut output = String::new();

        let fpr_registers = self
            .fpr
            .iter()
            .enumerate()
            .map(|(i, inst)| format!("fpr[{i}] = {inst}"))
            .collect::<Vec<String>>()
            .join("\n");
        output.push_str(&fpr_registers);

        output
    }
}

impl Index<&str> for FpRegisters {
    type Output = u64;

    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index(&self, index: &str) -> &Self::Output {
        match FpRegisterType::from_str(index) {
            Ok(register) => &self[register],
            _ => panic!("{index} is not a valid register"),
        }
    }
}

impl IndexMut<&str> for FpRegisters {
    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match FpRegisterType::from_str(index) {
            Ok(register) => &mut self[register],
            _ => panic!("{index} is not a valid register"),
        }
    }
}

impl Index<FpRegisterType> for FpRegisters {
    type Output = u64;

    fn index(&self, index: FpRegisterType) -> &Self::Output {
        match index {
            FpRegisterType::F0 => &self.fpr[0],
            FpRegisterType::F1 => &self.fpr[1],
            FpRegisterType::F2 => &self.fpr[2],
            FpRegisterType::F3 => &self.fpr[3],
            FpRegisterType::F4 => &self.fpr[4],
            FpRegisterType::F5 => &self.fpr[5],
            FpRegisterType::F6 => &self.fpr[6],
            FpRegisterType::F7 => &self.fpr[7],
            FpRegisterType::F8 => &self.fpr[8],
            FpRegisterType::F9 => &self.fpr[9],
            FpRegisterType::F10 => &self.fpr[10],
            FpRegisterType::F11 => &self.fpr[11],
            FpRegisterType::F12 => &self.fpr[12],
            FpRegisterType::F13 => &self.fpr[13],
            FpRegisterType::F14 => &self.fpr[14],
            FpRegisterType::F15 => &self.fpr[15],
            FpRegisterType::F16 => &self.fpr[16],
            FpRegisterType::F17 => &self.fpr[17],
            FpRegisterType::F18 => &self.fpr[18],
            FpRegisterType::F19 => &self.fpr[19],
            FpRegisterType::F20 => &self.fpr[20],
            FpRegisterType::F21 => &self.fpr[21],
            FpRegisterType::F22 => &self.fpr[22],
            FpRegisterType::F23 => &self.fpr[23],
            FpRegisterType::F24 => &self.fpr[24],
            FpRegisterType::F25 => &self.fpr[25],
            FpRegisterType::F26 => &self.fpr[26],
            FpRegisterType::F27 => &self.fpr[27],
            FpRegisterType::F28 => &self.fpr[28],
            FpRegisterType::F29 => &self.fpr[29],
            FpRegisterType::F30 => &self.fpr[30],
            FpRegisterType::F31 => &self.fpr[31],
        }
    }
}

impl IndexMut<FpRegisterType> for FpRegisters {
    fn index_mut(&mut self, index: FpRegisterType) -> &mut Self::Output {
        match index {
            FpRegisterType::F0 => &mut self.fpr[0],
            FpRegisterType::F1 => &mut self.fpr[1],
            FpRegisterType::F2 => &mut self.fpr[2],
            FpRegisterType::F3 => &mut self.fpr[3],
            FpRegisterType::F4 => &mut self.fpr[4],
            FpRegisterType::F5 => &mut self.fpr[5],
            FpRegisterType::F6 => &mut self.fpr[6],
            FpRegisterType::F7 => &mut self.fpr[7],
            FpRegisterType::F8 => &mut self.fpr[8],
            FpRegisterType::F9 => &mut self.fpr[9],
            FpRegisterType::F10 => &mut self.fpr[10],
            FpRegisterType::F11 => &mut self.fpr[11],
            FpRegisterType::F12 => &mut self.fpr[12],
            FpRegisterType::F13 => &mut self.fpr[13],
            FpRegisterType::F14 => &mut self.fpr[14],
            FpRegisterType::F15 => &mut self.fpr[15],
            FpRegisterType::F16 => &mut self.fpr[16],
            FpRegisterType::F17 => &mut self.fpr[17],
            FpRegisterType::F18 => &mut self.fpr[18],
            FpRegisterType::F19 => &mut self.fpr[19],
            FpRegisterType::F20 => &mut self.fpr[20],
            FpRegisterType::F21 => &mut self.fpr[21],
            FpRegisterType::F22 => &mut self.fpr[22],
            FpRegisterType::F23 => &mut self.fpr[23],
            FpRegisterType::F24 => &mut self.fpr[24],
            FpRegisterType::F25 => &mut self.fpr[25],
            FpRegisterType::F26 => &mut self.fpr[26],
            FpRegisterType::F27 => &mut self.fpr[27],
            FpRegisterType::F28 => &mut self.fpr[28],
            FpRegisterType::F29 => &mut self.fpr[29],
            FpRegisterType::F30 => &mut self.fpr[30],
            FpRegisterType::F31 => &mut self.fpr[31],
        }
    }
}

/// Iterator that is used to view each register in the register file.
///
/// This contains a copy of all the registers and their values, and a [`FpRegisterTypeIter`],
/// as generated by [`strum::IntoEnumIterator`]. In other iterator implementations,
/// the internal state might be data like a [`FpRegisterType`]. However, since we can't
/// normally just "add 1" to get to the next register, we use an internal iterator
/// that can track the progression of one [`FpRegisterType`] to the next.
pub struct FpRegistersIter {
    registers: FpRegisters,
    register_iter: FpRegisterTypeIter,
}

/// This implementation of the [`Iterator`] trait essentially wraps the existing
/// [`FpRegisterTypeIter`] so that the register type can be paired with register data.
impl Iterator for FpRegistersIter {
    type Item = (FpRegisterType, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.register_iter.next() {
            Some(register_type) => Some((register_type, self.registers[register_type])),
            None => None,
        }
    }
}

/// [`IntoIterator`] is a standard library trait that can convert any type into
/// an [`Iterator`]. In this case, this is an instance of [`FpRegistersIter`] with all the
/// data in the registers and a new [`FpRegisterTypeIter`].
impl IntoIterator for FpRegisters {
    type Item = (FpRegisterType, u64);
    type IntoIter = FpRegistersIter;

    /// Consumes the [`FpRegisters`] struct to create a new [`FpRegistersIter`] that can
    /// be iterated over.
    fn into_iter(self) -> Self::IntoIter {
        FpRegistersIter {
            registers: self,
            register_iter: FpRegisterType::iter(),
        }
    }
}
