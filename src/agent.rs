//! The agent responsible for running the emulator core on the worker thread and communication functionalities.

use crate::agent::messages::{Command, StateUpdate};
use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::{MipsDatapath, Stage};
use crate::emulation_core::mips::memory::Memory;
use crate::emulation_core::mips::registers::GpRegisterType;
use futures::{FutureExt, StreamExt};
use gloo_console::log;
use std::time::Duration;
use yew::platform::time::sleep;
use yew_agent::prelude::*;

pub mod datapath_communicator;
pub mod messages;

/// The main logic for the emulation core agent. All code within this function runs on a worker thread as opposed to
/// the UI thread.
#[reactor(EmulationCoreAgent)]
pub async fn emulation_core_agent(scope: ReactorScope<Command, StateUpdate>) {
    log!("Hello world!");
    let mut state = EmulatorCoreAgentState::new(scope);
    loop {
        let execution_delay = state.get_delay();
        futures::select! {
            // If we get a message, handle the command before attempting to execute.
            msg = state.scope.next() => match msg {
                Some(msg) => state.handle_command(msg),
                None => return,
            },
            // Delay to slow execution down to the intended speed.
            _ = sleep(Duration::from_millis(execution_delay)).fuse() => {},
        }

        // Execute a single instruction if the emulator core should be executing.
        state.execute();
    }
}

struct EmulatorCoreAgentState {
    current_datapath: Box<
        dyn Datapath<
            MemoryType = Memory,
            RegisterData = u64,
            RegisterEnum = GpRegisterType,
            StageEnum = Stage,
        >,
    >,
    pub scope: ReactorScope<Command, StateUpdate>,
    speed: u32,
    executing: bool,
}

impl EmulatorCoreAgentState {
    pub fn new(scope: ReactorScope<Command, StateUpdate>) -> EmulatorCoreAgentState {
        EmulatorCoreAgentState {
            current_datapath: Box::<MipsDatapath>::default(),
            scope,
            speed: 0,
            executing: false,
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::SetCore(_architecture) => {
                todo!() // Implement once we have a RISCV datapath
            }
            Command::LoadInstructions(_mem) => {
                // FIXME: Uncomment once refactoring is done on the trait
                // self.current_datapath.load_instructions(mem);
                todo!()
            }
            Command::SetExecuteSpeed(speed) => {
                self.speed = speed;
            }
            Command::SetRegister(_register, _value) => {
                todo!()
            }
            Command::SetMemory(_ptr, _data) => {
                // FIXME: Uncomment once refectoring is done on the trait
                // self.current_datapath.set_memory(ptr, &data);
                todo!()
            }
            Command::Execute => {
                self.executing = true;
            }
            Command::ExecuteInstruction => {
                self.current_datapath.execute_instruction();
            }
            Command::ExecuteStage => {
                self.current_datapath.execute_stage();
            }
            Command::Pause => {
                self.executing = false;
            }
        }
    }

    pub fn execute(&mut self) {
        // Skip the execution phase if the emulator core is not currently executing.
        if !self.executing {
            return;
        }
        self.current_datapath.execute_instruction();
    }

    /// Returns the delay between CPU cycles in milliseconds for the current execution speed. Will return zero if the
    /// execution speed is zero.
    pub fn get_delay(&self) -> u64 {
        if self.speed == 0 {
            0
        } else {
            (1000 / self.speed).into()
        }
    }
}
