use crate::emulation_core::mips::coprocessor::FpuState;
use crate::emulation_core::mips::datapath::DatapathState;
use crate::emulation_core::mips::fp_registers::FpRegisters;
use crate::emulation_core::mips::gp_registers::GpRegisters;
use crate::emulation_core::mips::memory::Memory;
use crate::emulation_core::riscv::datapath::{RiscDatapathState, RiscStage};
use crate::emulation_core::riscv::registers::RiscGpRegisters;
use crate::emulation_core::stack::Stack;
use crate::emulation_core::{architectures::AvailableDatapaths, mips::datapath::Stage};
use serde::{Deserialize, Serialize};

/// Commands sent from the UI thread to the worker thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    SetCore(AvailableDatapaths),
    Initialize(usize, Vec<u32>),
    SetExecuteSpeed(u32),
    SetRegister(String, u64),
    SetFPRegister(String, u64),
    SetMemory(u64, u32),
    Execute,
    ExecuteInstruction,
    ExecuteStage,
    Pause,
    Reset,
    SetBreakpoint(u64),
    RemoveBreakpoint(u64),
    Input(String),
}

/// Information about the emulator core's state sent from the worker thread to the UI thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MipsStateUpdate {
    UpdateState(DatapathState),
    UpdateRegisters(GpRegisters),
    UpdateCoprocessorState(FpuState),
    UpdateCoprocessorRegisters(FpRegisters),
    UpdateMemory(Memory),
    UpdateStage(Stage),
    UpdateStack(Stack),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RiscStateUpdate {
    UpdateState(RiscDatapathState),
    UpdateRegisters(RiscGpRegisters),
    UpdateMemory(Memory),
    UpdateStage(RiscStage),
    UpdateStack(Stack),
}

/// Information about the effects of system calls sent from the worker thread to the UI thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SystemUpdate {
    UpdateMessages(Vec<String>),
    UpdateExecuting(bool),
    UpdateInitialized(bool),
    UpdateSpeed(u32),
}

/// Enum containing all types of updates sent from the worker thread to the UI thread.
#[allow(clippy::large_enum_variant)] // Temporary allow while the System enum is tiny
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DatapathUpdate {
    MIPS(MipsStateUpdate),
    RISCV(RiscStateUpdate),
    System(SystemUpdate),
}
