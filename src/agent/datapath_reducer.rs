use crate::agent::messages::MipsStateUpdate;
use crate::emulation_core::architectures::AvailableDatapaths::MIPS;
use crate::emulation_core::architectures::{AvailableDatapaths, DatapathUpdate};
use crate::emulation_core::mips::coprocessor::MipsFpCoprocessor;
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

#[derive(Default, PartialEq)]
pub struct MipsCoreState {
    pub state: DatapathState,
    pub registers: GpRegisters,
    pub memory: Memory,
    pub current_stage: Stage,
    pub coprocessor: MipsFpCoprocessor,
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
                        current_stage: self.mips.current_stage.clone(),
                        coprocessor: self.mips.coprocessor.clone(),
                    },
                    MipsStateUpdate::UpdateRegisters(registers) => MipsCoreState {
                        state: self.mips.state.clone(),
                        registers,
                        memory: self.mips.memory.clone(),
                        current_stage: self.mips.current_stage.clone(),
                        coprocessor: self.mips.coprocessor.clone(),
                    },
                    MipsStateUpdate::UpdateMemory(memory) => MipsCoreState {
                        state: self.mips.state.clone(),
                        registers: self.mips.registers,
                        memory,
                        current_stage: self.mips.current_stage.clone(),
                        coprocessor: self.mips.coprocessor.clone(),
                    },
                    MipsStateUpdate::UpdateStage(stage) => MipsCoreState {
                        state: self.mips.state.clone(),
                        registers: self.mips.registers,
                        memory: self.mips.memory.clone(),
                        current_stage: stage,
                        coprocessor: self.mips.coprocessor.clone(),
                    },
                    MipsStateUpdate::UpdateCoprocessor(coprocessor) => MipsCoreState {
                        state: self.mips.state.clone(),
                        registers: self.mips.registers,
                        memory: self.mips.memory.clone(),
                        current_stage: self.mips.current_stage.clone(),
                        coprocessor,
                    },
                },
            },
        })
    }
}
