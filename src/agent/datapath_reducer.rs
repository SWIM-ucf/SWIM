use crate::agent::messages::MipsStateUpdate;
use crate::emulation_core::architectures::AvailableDatapaths::MIPS;
use crate::emulation_core::architectures::{AvailableDatapaths, DatapathUpdate};
use crate::emulation_core::mips::coprocessor::FpuState;
use crate::emulation_core::mips::datapath::{DatapathState, Stage};
use crate::emulation_core::mips::memory::Memory;
use crate::emulation_core::mips::registers::GpRegisters;
use std::rc::Rc;
use yew::Reducible;

#[derive(PartialEq)]
pub struct DatapathReducer {
    pub current_architecture: AvailableDatapaths,
    pub mips: MipsCoreState,
}

#[derive(Default, PartialEq, Clone)]
pub struct MipsCoreState {
    pub state: DatapathState,
    pub registers: GpRegisters,
    pub coprocessor_state: FpuState,
    pub coprocessor_registers: [u64; 32],
    pub memory: Memory,
    pub current_stage: Stage,
}

impl Default for DatapathReducer {
    fn default() -> Self {
        Self {
            current_architecture: MIPS,
            mips: MipsCoreState::default(),
        }
    }
}

impl Reducible for DatapathReducer {
    type Action = DatapathUpdate;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
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
            },
        })
    }
}
