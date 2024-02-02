use crate::emulation_core::architectures::AvailableDatapaths;
use serde::{Deserialize, Serialize};

/// Commands sent from the UI thread to the worker thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    SetCore(AvailableDatapaths),
    LoadInstructions(Vec<u8>),
    SetExecuteSpeed(u32),
    SetRegister(String, u64),
    SetMemory(usize, Vec<u8>),
    Execute,
    ExecuteInstruction,
    ExecuteStage,
    Pause,
}

/// Information about the emulator core's state sent from the worker thread to the UI thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StateUpdate {
    UpdateRegister(u64),
    UpdateMemory(usize, Vec<u8>),
    SetCurrentStage(String),
    SetCurrentInstruction(usize),
    AddMemorySegment,
    RemoveMemorySegment,
}
