use crate::emulation_core::architectures::AvailableDatapaths;
use crate::emulation_core::mips::datapath::DatapathState;
use crate::emulation_core::mips::memory::Memory;
use crate::emulation_core::mips::registers::GpRegisters;
use serde::{Deserialize, Serialize};

/// Commands sent from the UI thread to the worker thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    SetCore(AvailableDatapaths),
    Initialize(usize, Vec<u8>),
    SetExecuteSpeed(u32),
    SetRegister(String, u64),
    SetMemory(u64, u32),
    Execute,
    ExecuteInstruction,
    ExecuteStage,
    Pause,
    Reset,
}

/// Information about the emulator core's state sent from the worker thread to the UI thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MipsStateUpdate {
    UpdateState(DatapathState),
    UpdateRegisters(GpRegisters),
    UpdateMemory(Memory),
}
