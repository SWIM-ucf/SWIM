use crate::agent::messages::MipsStateUpdate;
use crate::emulation_core::architectures::AvailableDatapaths::MIPS;
use crate::emulation_core::architectures::{AvailableDatapaths, DatapathUpdate};
use crate::emulation_core::mips::datapath::DatapathState;
use crate::emulation_core::mips::memory::Memory;
use crate::emulation_core::mips::registers::GpRegisters;
use std::rc::Rc;
use yew::Reducible;

pub struct DatapathReducer {
    pub current_architecture: AvailableDatapaths,
    pub mips: MipsCoreState,
}

#[derive(Default)]
pub struct MipsCoreState {
    pub state: DatapathState,
    pub registers: GpRegisters,
    pub memory: Memory,
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
                        registers: self.mips.registers,
                        memory: self.mips.memory.clone(),
                    },
                    MipsStateUpdate::UpdateRegisters(registers) => MipsCoreState {
                        state: self.mips.state.clone(),
                        registers,
                        memory: self.mips.memory.clone(),
                    },
                    MipsStateUpdate::UpdateMemory(memory) => MipsCoreState {
                        state: self.mips.state.clone(),
                        registers: self.mips.registers,
                        memory,
                    },
                },
            },
        })
    }
}
