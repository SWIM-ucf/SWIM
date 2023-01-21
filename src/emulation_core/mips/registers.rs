//! Register structure and API.

use std::ops::{Index, IndexMut};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

/// Collection of general-purpose registers used by the datapath.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct GpRegisters {
    pub pc: u64,
    pub gpr: [u64; 32],
}

/// Specifies all of the valid registers accessible in an instance
/// of [`GpRegisters`].
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum GpRegisterType {
    Pc,
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}

impl Index<&str> for GpRegisters {
    type Output = u64;

    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index(&self, index: &str) -> &Self::Output {
        match GpRegisterType::from_str(index) {
            Ok(register) => &self[register],
            _ => panic!("{} is not a valid register", index),
        }
    }
}

impl IndexMut<&str> for GpRegisters {
    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match GpRegisterType::from_str(index) {
            Ok(register) => &mut self[register],
            _ => panic!("{} is not a valid register", index),
        }
    }
}

impl Index<GpRegisterType> for GpRegisters {
    type Output = u64;

    fn index(&self, index: GpRegisterType) -> &Self::Output {
        match index {
            GpRegisterType::Pc => &self.pc,
            GpRegisterType::Zero => &self.gpr[0],
            GpRegisterType::At => &self.gpr[1],
            GpRegisterType::V0 => &self.gpr[2],
            GpRegisterType::V1 => &self.gpr[3],
            GpRegisterType::A0 => &self.gpr[4],
            GpRegisterType::A1 => &self.gpr[5],
            GpRegisterType::A2 => &self.gpr[6],
            GpRegisterType::A3 => &self.gpr[7],
            GpRegisterType::T0 => &self.gpr[8],
            GpRegisterType::T1 => &self.gpr[9],
            GpRegisterType::T2 => &self.gpr[10],
            GpRegisterType::T3 => &self.gpr[11],
            GpRegisterType::T4 => &self.gpr[12],
            GpRegisterType::T5 => &self.gpr[13],
            GpRegisterType::T6 => &self.gpr[14],
            GpRegisterType::T7 => &self.gpr[15],
            GpRegisterType::S0 => &self.gpr[16],
            GpRegisterType::S1 => &self.gpr[17],
            GpRegisterType::S2 => &self.gpr[18],
            GpRegisterType::S3 => &self.gpr[19],
            GpRegisterType::S4 => &self.gpr[20],
            GpRegisterType::S5 => &self.gpr[21],
            GpRegisterType::S6 => &self.gpr[22],
            GpRegisterType::S7 => &self.gpr[23],
            GpRegisterType::T8 => &self.gpr[24],
            GpRegisterType::T9 => &self.gpr[25],
            GpRegisterType::K0 => &self.gpr[26],
            GpRegisterType::K1 => &self.gpr[27],
            GpRegisterType::Gp => &self.gpr[28],
            GpRegisterType::Sp => &self.gpr[29],
            GpRegisterType::Fp => &self.gpr[30],
            GpRegisterType::Ra => &self.gpr[31],
        }
    }
}

impl IndexMut<GpRegisterType> for GpRegisters {
    fn index_mut(&mut self, index: GpRegisterType) -> &mut Self::Output {
        match index {
            GpRegisterType::Pc => &mut self.pc,
            GpRegisterType::Zero => panic!("The $zero register cannot be accessed as mutable"),
            GpRegisterType::At => &mut self.gpr[1],
            GpRegisterType::V0 => &mut self.gpr[2],
            GpRegisterType::V1 => &mut self.gpr[3],
            GpRegisterType::A0 => &mut self.gpr[4],
            GpRegisterType::A1 => &mut self.gpr[5],
            GpRegisterType::A2 => &mut self.gpr[6],
            GpRegisterType::A3 => &mut self.gpr[7],
            GpRegisterType::T0 => &mut self.gpr[8],
            GpRegisterType::T1 => &mut self.gpr[9],
            GpRegisterType::T2 => &mut self.gpr[10],
            GpRegisterType::T3 => &mut self.gpr[11],
            GpRegisterType::T4 => &mut self.gpr[12],
            GpRegisterType::T5 => &mut self.gpr[13],
            GpRegisterType::T6 => &mut self.gpr[14],
            GpRegisterType::T7 => &mut self.gpr[15],
            GpRegisterType::S0 => &mut self.gpr[16],
            GpRegisterType::S1 => &mut self.gpr[17],
            GpRegisterType::S2 => &mut self.gpr[18],
            GpRegisterType::S3 => &mut self.gpr[19],
            GpRegisterType::S4 => &mut self.gpr[20],
            GpRegisterType::S5 => &mut self.gpr[21],
            GpRegisterType::S6 => &mut self.gpr[22],
            GpRegisterType::S7 => &mut self.gpr[23],
            GpRegisterType::T8 => &mut self.gpr[24],
            GpRegisterType::T9 => &mut self.gpr[25],
            GpRegisterType::K0 => &mut self.gpr[26],
            GpRegisterType::K1 => &mut self.gpr[27],
            GpRegisterType::Gp => &mut self.gpr[28],
            GpRegisterType::Sp => &mut self.gpr[29],
            GpRegisterType::Fp => &mut self.gpr[30],
            GpRegisterType::Ra => &mut self.gpr[31],
        }
    }
}

/// Iterator that is used to view each register in the register file.
///
/// This contains a copy of all the registers and their values, and a [`GpRegisterTypeIter`],
/// as generated by [`strum::IntoEnumIterator`]. In other iterator implementations,
/// the internal state might be data like a [`GpRegisterType`]. However, since we can't
/// normally just "add 1" to get to the next register, we use an internal iterator
/// that can track the progression of one [`GpRegisterType`] to the next.
pub struct GpRegistersIter {
    registers: GpRegisters,
    register_iter: GpRegisterTypeIter,
}

/// This implementation of the [`Iterator`] trait essentially wraps the existing
/// [`GpRegisterTypeIter`] so that the register type can be paired with register data.
impl Iterator for GpRegistersIter {
    type Item = (GpRegisterType, u64);

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
impl IntoIterator for GpRegisters {
    type Item = (GpRegisterType, u64);
    type IntoIter = GpRegistersIter;

    /// Consumes the [`GpRegisters`] struct to create a new [`GpRegistersIter`] that can
    /// be iterated over.
    fn into_iter(self) -> Self::IntoIter {
        GpRegistersIter {
            registers: self,
            register_iter: GpRegisterType::iter(),
        }
    }
}
