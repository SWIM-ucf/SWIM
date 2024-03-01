//! The agent responsible for running the emulator core on the worker thread and communication functionalities.

use crate::agent::messages::Command;
use crate::agent::messages::MipsStateUpdate::*;
use crate::emulation_core::architectures::{DatapathRef, DatapathUpdate};
use crate::emulation_core::datapath::{Datapath, DatapathUpdateSignal, UPDATE_EVERYTHING};
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::registers::GpRegisterType;
use futures::{FutureExt, SinkExt, StreamExt};
use gloo_console::log;
use std::time::Duration;
use yew::platform::time::sleep;
use yew_agent::prelude::*;

pub mod datapath_communicator;
pub mod datapath_reducer;
pub mod messages;

macro_rules! send_update {
    ($scope:expr, $condition:expr, $value:expr) => {
        if $condition {
            $scope
                .send($value)
                .await
                .expect("ReactorScope's send() function should not fail.")
        }
    };
}

macro_rules! send_update_mips {
    ($scope:expr, $cond:expr, $data:expr) => {
        send_update!($scope, $cond, DatapathUpdate::MIPS($data))
    };
}

/// The main logic for the emulation core agent. All code within this function runs on a worker thread as opposed to
/// the UI thread.
#[reactor(EmulationCoreAgent)]
pub async fn emulation_core_agent(scope: ReactorScope<Command, DatapathUpdate>) {
    log!("Hello world!");
    let mut state = EmulatorCoreAgentState::new(scope);
    loop {
        let execution_delay = state.get_delay();

        // Part 1: Delay/Command Handling
        if state.executing {
            futures::select! {
                // If we get a message, handle the command before attempting to execute.
                msg = state.scope.next() => match msg {
                    Some(msg) => state.handle_command(msg),
                    None => return,
                },
                // Delay to slow execution down to the intended speed.
                _ = sleep(Duration::from_millis(execution_delay)).fuse() => {},
            }
        } else {
            // If we're not currently executing, wait indefinitely until the next message comes in.
            match state.scope.next().await {
                Some(msg) => state.handle_command(msg),
                None => return,
            }
        }

        // Part 2: Execution
        // Execute a single instruction if the emulator core should be executing.
        state.execute();

        // Part 3: Processing State/Sending Updates to UI
        match state.current_datapath.as_datapath_ref() {
            DatapathRef::MIPS(datapath) => {
                log!(format!("Updates: {:?}", state.updates));
                // Stage always updates
                send_update_mips!(state.scope, true, UpdateStage(datapath.current_stage));

                // Send all other updates based on the state.updates variable.
                send_update_mips!(
                    state.scope,
                    state.updates.changed_state,
                    UpdateState(datapath.state.clone())
                );
                send_update_mips!(
                    state.scope,
                    state.updates.changed_registers,
                    UpdateRegisters(datapath.registers)
                );
                send_update_mips!(
                    state.scope,
                    state.updates.changed_coprocessor_state,
                    UpdateCoprocessorState(datapath.coprocessor.state.clone())
                );
                send_update_mips!(
                    state.scope,
                    state.updates.changed_coprocessor_registers,
                    UpdateCoprocessorRegisters(datapath.coprocessor.fpr)
                );
                send_update_mips!(
                    state.scope,
                    state.updates.changed_memory,
                    UpdateMemory(datapath.memory.clone())
                );
            }
        }
        state.updates = Default::default();
    }
}

struct EmulatorCoreAgentState {
    current_datapath: Box<dyn Datapath<RegisterData = u64, RegisterEnum = GpRegisterType>>,
    /// The changes to the emulator core's memory/registers/etc. are tracked in this variable. When
    /// it's time to send updates back to the main thread, this variable determines which updates
    /// get sent.
    pub updates: DatapathUpdateSignal,
    pub scope: ReactorScope<Command, DatapathUpdate>,
    speed: u32,
    executing: bool,
}

impl EmulatorCoreAgentState {
    pub fn new(scope: ReactorScope<Command, DatapathUpdate>) -> EmulatorCoreAgentState {
        EmulatorCoreAgentState {
            current_datapath: Box::<MipsDatapath>::default(),
            updates: DatapathUpdateSignal::default(),
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
            Command::Initialize(initial_pc, mem) => {
                self.current_datapath.initialize(initial_pc, mem).unwrap();
                self.updates.changed_memory = true;
                self.updates.changed_registers = true;
            }
            Command::SetExecuteSpeed(speed) => {
                self.speed = speed;
            }
            Command::SetRegister(register, value) => {
                self.current_datapath.set_register_by_str(&register, value);
                self.updates.changed_registers = true;
            }
            Command::SetMemory(ptr, data) => {
                self.current_datapath.set_memory(ptr, data);
                self.updates.changed_memory = true;
            }
            Command::Execute => {
                self.executing = true;
            }
            Command::ExecuteInstruction => {
                self.updates |= self.current_datapath.execute_instruction();
            }
            Command::ExecuteStage => {
                self.updates |= self.current_datapath.execute_stage();
            }
            Command::Pause => {
                self.executing = false;
            }
            Command::Reset => {
                self.current_datapath.reset();
                self.updates |= UPDATE_EVERYTHING;
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
