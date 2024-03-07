use crate::agent::messages::{DatapathUpdate, MipsStateUpdate, SystemUpdate};
use crate::emulation_core::architectures::AvailableDatapaths;
use crate::emulation_core::architectures::AvailableDatapaths::MIPS;
use crate::emulation_core::mips::coprocessor::FpuState;
use crate::emulation_core::mips::datapath::{DatapathState, Stage};
use crate::emulation_core::mips::fp_registers::FpRegisters;
use crate::emulation_core::mips::gp_registers::GpRegisters;
use crate::emulation_core::mips::memory::Memory;
use gloo_console::log;
use std::rc::Rc;
use yew::Reducible;

#[derive(PartialEq)]
pub struct DatapathReducer {
    pub current_architecture: AvailableDatapaths,
    pub mips: MipsCoreState,
    pub messages: Vec<String>,
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

impl Default for DatapathReducer {
    fn default() -> Self {
        Self {
            current_architecture: MIPS,
            mips: MipsCoreState::default(),
            messages: Vec::new(),
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
                messages: self.messages.clone(),
            },
            DatapathUpdate::System(update) => match update {
                SystemUpdate::Message(msg) => Self {
                    current_architecture: self.current_architecture.clone(),
                    mips: self.mips.clone(),
                    messages: {
                        let mut new_messages = self.messages.clone();
                        new_messages.push(msg);
                        new_messages
                    },
                },
            },
        })
    }
}
