//! The agent responsible for running the emulator core on the worker thread and communication functionalities.

use crate::agent::messages::{Command, MipsStateUpdate};
use crate::emulation_core::architectures::{DatapathRef, DatapathUpdate};
use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::gp_registers::GpRegisterType;
use futures::{FutureExt, SinkExt, StreamExt};
use gloo_console::log;
use std::time::Duration;
use yew::platform::time::sleep;
use yew_agent::prelude::*;

pub mod datapath_communicator;
pub mod datapath_reducer;
pub mod messages;

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
        // TODO: This is a very naive implementation. Optimization is probably a good idea.
        // TODO: Add support for the FP coprocessor updates in MIPS
        match state.current_datapath.as_datapath_ref() {
            DatapathRef::MIPS(datapath) => {
                let state_update =
                    DatapathUpdate::MIPS(MipsStateUpdate::UpdateState(datapath.state.clone()));
                let register_update =
                    DatapathUpdate::MIPS(MipsStateUpdate::UpdateRegisters(datapath.registers));
                let memory_update =
                    DatapathUpdate::MIPS(MipsStateUpdate::UpdateMemory(datapath.memory.clone()));
                let stage_update = DatapathUpdate::MIPS(MipsStateUpdate::UpdateStage(
                    datapath.current_stage,
                ));
                let coprocessor_update = DatapathUpdate::MIPS(MipsStateUpdate::UpdateCoprocessor(
                    datapath.coprocessor.clone(),
                ));
                state.scope.send(state_update).await.unwrap();
                state.scope.send(register_update).await.unwrap();
                state.scope.send(memory_update).await.unwrap();
                state.scope.send(stage_update).await.unwrap();
                state.scope.send(coprocessor_update).await.unwrap();
            }
        }
    }
}

struct EmulatorCoreAgentState {
    current_datapath: Box<dyn Datapath<RegisterData = u64, RegisterEnum = GpRegisterType>>,
    pub scope: ReactorScope<Command, DatapathUpdate>,
    speed: u32,
    executing: bool,
}

impl EmulatorCoreAgentState {
    pub fn new(scope: ReactorScope<Command, DatapathUpdate>) -> EmulatorCoreAgentState {
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
            Command::Initialize(initial_pc, mem) => {
                self.current_datapath.initialize(initial_pc, mem).unwrap();
            }
            Command::SetExecuteSpeed(speed) => {
                self.speed = speed;
            }
            Command::SetRegister(register, value) => {
                self.current_datapath.set_register_by_str(&register, value);
            }
            Command::SetFPRegister(register, value) => {
                self.current_datapath
                    .set_fp_register_by_str(&register, value);
            }
            Command::SetMemory(ptr, data) => {
                self.current_datapath.set_memory(ptr, data);
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
            Command::Reset => {
                self.current_datapath.reset();
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
