//! Module for the API of a generic datapath.

use crate::emulation_core::architectures::DatapathRef;
use crate::emulation_core::mips::line_info::LineInformation;
use crate::emulation_core::mips::memory::Memory;

/// A generic datapath.
///
/// This has the ability to execute instructions, and to interface with
/// registers and memory. The generic datapath interface additionally
/// specifies a series of data types that correspond to the types of
/// inputs and outputs associated with that datapath.
///
/// This design allows developers to create their own datapaths for
/// other architectures than the one created for the sake of this
/// project ([`MipsDatapath`](super::mips::datapath::MipsDatapath)).
pub trait Datapath {
    /// The type of data stored within registers. (Suggestions may
    /// include [`u16`], [`u32`], or [`u64`] for 16-bit, 32-bit, or 64-bit
    /// registers, respectively.)
    type RegisterData;

    /// The enum used to describe all available registers used in the
    /// datapath. This must be defined separately, and at minimum simply
    /// contain a list of registers. Further implementation details are
    /// at the discretion of the developer.
    type RegisterEnum;

    /// Execute a single instruction based on the current state of the
    /// datapath. Should the datapath support stages, if the datapath is
    /// midway through a stage, the current instruction will be finished
    /// instead of executing a new instruction. Should the datapath be in
    /// a "halted" state, behavior is undefined.
    fn execute_instruction(&mut self);

    /// Execute a single stage of execution based on the current state of
    /// the datapath. Should the datapath not support stages, assume the
    /// same behavior as [`Self::execute_instruction()`]. Should the
    /// datapath be in a "halted" state, behavior is undefined.
    fn execute_stage(&mut self);

    /// Retrieve the data in the register indicated by the provided enum.
    /// It can be assumed valid data will be retrieved since any valid
    /// registers should be listed within [`Self::RegisterEnum`].
    fn get_register_by_enum(&self, register: Self::RegisterEnum) -> Self::RegisterData;

    /// Sets the data in the register indicated by the provided string. If it doesn't exist,
    /// this function returns Err.
    fn set_register_by_str(&mut self, register: &str, data: Self::RegisterData);

    /// Reset the datapath, load instructions into memory, and un-sets the `is_halted`
    /// flag. If the process fails, an [`Err`] is returned.
    fn initialize(&mut self, instructions: Vec<u8>) -> Result<(), String>;

    /// Retrieve all memory as-is.
    fn get_memory(&self) -> &Memory;

    fn set_memory(&mut self, _ptr: usize, _data: &[u8]) {
        todo!()
    }

    /// Returns if the datapath is in a "halted" or "stopped" state. This may
    /// be true in the case where an error had occurred previously.
    fn is_halted(&self) -> bool;

    /// Restore the datapath to its default state.
    fn reset(&mut self);

    fn as_datapath_ref(&self) -> DatapathRef;
}

/// A datapath that supports a visual diagram component.
///
/// This requires a corresponding visual diagram with labels that can be mapped
/// to the datapath.
pub trait VisualDatapath {
    /// Return the information from the datapath corresponding to the `variable` attribute on a
    /// part of the visual datapath diagram.
    fn visual_line_to_data(&self, variable: &str) -> LineInformation;
}
