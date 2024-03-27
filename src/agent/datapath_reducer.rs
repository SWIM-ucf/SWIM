use crate::agent::messages::{DatapathUpdate, MipsStateUpdate, RiscStateUpdate, SystemUpdate};
use crate::emulation_core::architectures::AvailableDatapaths;
use crate::emulation_core::architectures::AvailableDatapaths::{MIPS, RISCV};
use crate::emulation_core::mips::coprocessor::FpuState;
use crate::emulation_core::mips::datapath::{DatapathState, Stage};
use crate::emulation_core::mips::fp_registers::FpRegisters;
use crate::emulation_core::mips::gp_registers::GpRegisters;
use crate::emulation_core::mips::memory::Memory;
use crate::emulation_core::register::{RegisterType, Registers};
use crate::emulation_core::riscv::datapath::{RiscDatapathState, RiscStage};
use crate::emulation_core::riscv::registers::RiscGpRegisters;
use gloo_console::log;
use std::rc::Rc;
use yew::Reducible;

#[derive(PartialEq, Clone)]
pub struct DatapathReducer {
    pub current_architecture: AvailableDatapaths,
    pub mips: MipsCoreState,
    pub riscv: RiscCoreState,
    pub messages: Vec<String>,
    pub speed: u32,
    pub executing: bool,
    pub initialized: bool,
}

#[derive(Default, PartialEq, Clone)]
pub struct MipsCoreState {
    pub state: DatapathState,
    pub registers: GpRegisters,
    pub coprocessor_state: FpuState,
    pub coprocessor_registers: FpRegisters,
    pub memory: Memory,
    pub current_stage: Stage,
}

#[derive(Default, PartialEq, Clone)]
pub struct RiscCoreState {
    pub state: RiscDatapathState,
    pub registers: RiscGpRegisters,
    pub memory: Memory,
    pub current_stage: RiscStage,
}

impl Default for DatapathReducer {
    fn default() -> Self {
        Self {
            current_architecture: MIPS,
            mips: MipsCoreState::default(),
            riscv: RiscCoreState::default(),
            messages: Vec::new(),
            speed: 0,
            executing: false,
            initialized: false,
        }
    }
}

impl Reducible for DatapathReducer {
    type Action = DatapathUpdate;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        log!("Messages so far:");
        for item in &self.messages {
            log!(item);
        }

        Rc::from(match action {
            DatapathUpdate::MIPS(update) => Self {
                current_architecture: MIPS,
                mips: match update {
                    MipsStateUpdate::UpdateState(state) => MipsCoreState {
                        state,
                        ..self.mips.clone()
                    },
                    MipsStateUpdate::UpdateRegisters(registers) => MipsCoreState {
                        registers,
                        ..self.mips.clone()
                    },
                    MipsStateUpdate::UpdateCoprocessorState(coprocessor_state) => MipsCoreState {
                        coprocessor_state,
                        ..self.mips.clone()
                    },
                    MipsStateUpdate::UpdateCoprocessorRegisters(coprocessor_registers) => {
                        MipsCoreState {
                            coprocessor_registers,
                            ..self.mips.clone()
                        }
                    }
                    MipsStateUpdate::UpdateMemory(memory) => MipsCoreState {
                        memory,
                        ..self.mips.clone()
                    },
                    MipsStateUpdate::UpdateStage(stage) => MipsCoreState {
                        current_stage: stage,
                        ..self.mips.clone()
                    },
                },
                ..(*self).clone()
            },
            DatapathUpdate::System(update) => match update {
                SystemUpdate::UpdateMessages(messages) => Self {
                    messages,
                    ..(*self).clone()
                },
                SystemUpdate::UpdateExecuting(executing) => Self {
                    executing,
                    ..(*self).clone()
                },
                SystemUpdate::UpdateInitialized(initialized) => Self {
                    initialized,
                    ..(*self).clone()
                },
            },
            DatapathUpdate::RISCV(update) => Self {
                current_architecture: RISCV,
                riscv: match update {
                    RiscStateUpdate::UpdateState(state) => RiscCoreState {
                        state,
                        ..self.riscv.clone()
                    },
                    RiscStateUpdate::UpdateRegisters(registers) => RiscCoreState {
                        registers,
                        ..self.riscv.clone()
                    },
                    RiscStateUpdate::UpdateMemory(memory) => RiscCoreState {
                        memory,
                        ..self.riscv.clone()
                    },
                    RiscStateUpdate::UpdateStage(current_stage) => RiscCoreState {
                        current_stage,
                        ..self.riscv.clone()
                    },
                },
                ..(*self).clone()
            },
        })
    }
}

impl DatapathReducer {
    pub fn get_pc(&self) -> u64 {
        match self.current_architecture {
            MIPS => self.mips.registers.pc,
            RISCV => self.riscv.registers.pc,
        }
    }

    pub fn get_dyn_gp_registers(&self) -> Vec<(Rc<dyn RegisterType>, u64)> {
        match self.current_architecture {
            MIPS => self.mips.registers.get_dyn_register_list(),
            RISCV => self.riscv.registers.get_dyn_register_list(),
        }
    }

    pub fn get_memory(&self) -> &Memory {
        match self.current_architecture {
            MIPS => &self.mips.memory,
            RISCV => &self.riscv.memory,
        }
    }

    pub fn get_current_stage(&self) -> String {
        match self.current_architecture {
            MIPS => self.mips.current_stage.into(),
            RISCV => self.riscv.current_stage.into(),
        }
    }
}
