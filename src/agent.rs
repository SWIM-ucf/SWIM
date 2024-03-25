//! The agent responsible for running the emulator core on the worker thread and communication functionalities.

use crate::agent::messages::MipsStateUpdate::*;
use crate::agent::messages::{Command, SystemUpdate};
use crate::agent::system_scanner::Scanner;
use crate::emulation_core::architectures::DatapathRef;
use crate::emulation_core::datapath::{Datapath, DatapathUpdateSignal, Syscall, UPDATE_EVERYTHING};
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::gp_registers::GpRegisterType;
use futures::{FutureExt, SinkExt, StreamExt};
use gloo_console::log;
use messages::DatapathUpdate;
use std::collections::HashSet;
use std::time::Duration;
use yew::platform::time::sleep;
use yew_agent::prelude::*;

pub mod datapath_communicator;
pub mod datapath_reducer;
pub mod messages;
pub mod system_scanner;

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

        // Save the previous state of the emulator core's execution and initialization status
        let is_executing = state.executing;
        let is_initialiized = state.initialized;

        // Part 1: Delay/Command Handling
        if state.executing {
            futures::select! {
                // If we get a message, handle the command before attempting to execute.
                msg = state.scope.next() => match msg {
                    Some(msg) => state.handle_command(msg).await,
                    None => return,
                },
                // Delay to slow execution down to the intended speed.
                _ = sleep(Duration::from_millis(execution_delay)).fuse() => {},
            }
        } else {
            // If we're not currently executing, wait indefinitely until the next message comes in.
            match state.scope.next().await {
                Some(msg) => state.handle_command(msg).await,
                None => return,
            }
        }

        // Part 2: Execution
        // Execute a single instruction if the emulator core should be executing.
        state.execute();

        // Part 3: Performing Syscalls
        state.execute_syscall_stage().await;

        // Part 4: Processing State/Sending Updates to UI
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
                    UpdateCoprocessorRegisters(datapath.coprocessor.registers)
                );
                send_update_mips!(
                    state.scope,
                    state.updates.changed_memory,
                    UpdateMemory(datapath.memory.clone())
                );
            }
        }
        // Part 5: Sending Non-Syscall System Updates to UI
        send_update!(
            state.scope,
            state.executing != is_executing,
            DatapathUpdate::System(SystemUpdate::UpdateExecuting(state.executing))
        );
        send_update!(
            state.scope,
            state.initialized != is_initialiized,
            DatapathUpdate::System(SystemUpdate::UpdateInitialized(state.initialized))
        );
        state.updates = Default::default();
    }
}

#[derive(Clone, PartialEq, Default)]
enum BlockedOn {
    #[default]
    Nothing,
    Syscall(Syscall),
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
    initialized: bool,
    messages: Vec<String>,
    scanner: Scanner,
    blocked_on: BlockedOn,
    breakpoints: HashSet<u64>,
}

impl EmulatorCoreAgentState {
    pub fn new(scope: ReactorScope<Command, DatapathUpdate>) -> EmulatorCoreAgentState {
        EmulatorCoreAgentState {
            current_datapath: Box::<MipsDatapath>::default(),
            updates: DatapathUpdateSignal::default(),
            scope,
            speed: 0,
            executing: false,
            initialized: false,
            messages: Vec::new(),
            scanner: Scanner::new(),
            blocked_on: BlockedOn::Nothing,
            breakpoints: HashSet::default(),
        }
    }

    pub async fn handle_command(&mut self, command: Command) {
        match command {
            Command::SetCore(_architecture) => {
                todo!("Implement setting cores.") // Implement once we have a RISCV datapath
            }
            Command::Initialize(initial_pc, mem) => {
                self.current_datapath.initialize(initial_pc, mem).unwrap();
                self.reset_system().await;
                self.updates |= UPDATE_EVERYTHING;
                self.initialized = true;
            }
            Command::SetExecuteSpeed(speed) => {
                self.speed = speed;
            }
            Command::SetRegister(register, value) => {
                self.current_datapath.set_register_by_str(&register, value);
                self.updates.changed_registers = true;
            }
            Command::SetFPRegister(register, value) => {
                self.current_datapath
                    .set_fp_register_by_str(&register, value);
            }
            Command::SetMemory(ptr, data) => {
                self.current_datapath.set_memory(ptr, data);
                self.updates.changed_memory = true;
            }
            Command::Execute => {
                self.executing = true;
            }
            Command::ExecuteInstruction => {
                if self.blocked_on == BlockedOn::Nothing {
                    self.updates |= self.current_datapath.execute_instruction();
                }
            }
            Command::ExecuteStage => {
                if self.blocked_on == BlockedOn::Nothing {
                    self.updates |= self.current_datapath.execute_stage();
                }
            }
            Command::Pause => {
                self.executing = false;
            }
            Command::Reset => {
                self.current_datapath.reset();
                self.reset_system().await;
                self.updates |= UPDATE_EVERYTHING;
            }
            Command::Input(line) => {
                self.add_message(format!("> {}", line)).await;
                self.scanner.feed(line);
            }
            Command::SetBreakpoint(address) => {
                self.breakpoints.insert(address);
            }
            Command::RemoveBreakpoint(address) => {
                self.breakpoints.remove(&address);
            }
        }
    }

    pub fn execute(&mut self) {
        // Skip the execution phase if the emulator core is not currently executing.
        if !self.executing || matches!(self.blocked_on, BlockedOn::Syscall(_)) {
            return;
        }
        self.updates |= self.current_datapath.execute_instruction();

        // Extract the current program counter and break if there's a breakpoint set here.
        let current_pc = match self.current_datapath.as_datapath_ref() {
            DatapathRef::MIPS(datapath) => datapath.registers.pc,
        };
        if self.breakpoints.contains(&current_pc) {
            self.executing = false;
        }
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

    pub async fn execute_syscall_stage(&mut self) {
        if !self.updates.hit_syscall && !matches!(self.blocked_on, BlockedOn::Syscall(_)) {
            return;
        }

        // Determine if we should attempt to execute a new syscall or poll on a previous syscall
        // the processor blocked on.
        let syscall = match &self.blocked_on {
            BlockedOn::Nothing => self.current_datapath.get_syscall_arguments(),
            BlockedOn::Syscall(syscall) => syscall.clone(),
        };

        match syscall {
            Syscall::Exit => {
                self.current_datapath.halt();
            }
            Syscall::PrintInt(val) => {
                self.add_message(val.to_string()).await;
            }
            Syscall::PrintFloat(val) => {
                self.add_message(val.to_string()).await;
            }
            Syscall::PrintDouble(val) => {
                self.add_message(val.to_string()).await;
            }
            Syscall::PrintString(val) => {
                self.add_message(val.to_string()).await;
            }
            Syscall::ReadInt => {
                let scan_result = self.scanner.next_int();
                match scan_result {
                    None => {
                        self.blocked_on = BlockedOn::Syscall(syscall);
                    }
                    Some(scan_result) => {
                        self.blocked_on = BlockedOn::Nothing;
                        match self.current_datapath.as_datapath_ref() {
                            DatapathRef::MIPS(_) => {
                                self.current_datapath.set_register_by_str("v0", scan_result);
                            }
                        }
                        self.updates.changed_registers = true;
                    }
                }
            }
            Syscall::ReadFloat => {
                let scan_result = self.scanner.next_float();
                match scan_result {
                    None => {
                        self.blocked_on = BlockedOn::Syscall(syscall);
                    }
                    Some(scan_result) => {
                        self.blocked_on = BlockedOn::Nothing;
                        match self.current_datapath.as_datapath_ref() {
                            DatapathRef::MIPS(_) => {
                                self.current_datapath
                                    .set_fp_register_by_str("f0", f32::to_bits(scan_result) as u64);
                            }
                        }
                        self.updates.changed_coprocessor_registers = true;
                    }
                }
            }
            Syscall::ReadDouble => {
                let scan_result = self.scanner.next_double();
                match scan_result {
                    None => {
                        self.blocked_on = BlockedOn::Syscall(syscall);
                    }
                    Some(scan_result) => {
                        self.blocked_on = BlockedOn::Nothing;
                        match self.current_datapath.as_datapath_ref() {
                            DatapathRef::MIPS(_) => {
                                self.current_datapath
                                    .set_fp_register_by_str("f0", f64::to_bits(scan_result));
                            }
                        }
                        self.updates.changed_coprocessor_registers = true;
                    }
                }
            }
            Syscall::ReadString(addr) => {
                let scan_result = self.scanner.next_line();
                match scan_result {
                    None => {
                        self.blocked_on = BlockedOn::Syscall(syscall);
                    }
                    Some(scan_result) => {
                        self.blocked_on = BlockedOn::Nothing;

                        let bytes = scan_result.as_bytes();
                        let memory = self.current_datapath.get_memory_mut();
                        let mut failed_store = false;
                        for (i, byte) in bytes.iter().enumerate() {
                            // Attempt to store the byte in memory, but if the store process fails,
                            // end the syscall and return to normal operation.
                            let result = memory.store_byte(addr + i as u64, *byte);
                            if result.is_err() {
                                failed_store = true;
                                break;
                            }
                        }
                        match self.current_datapath.as_datapath_ref() {
                            DatapathRef::MIPS(_) => {
                                if failed_store {
                                    self.current_datapath.set_register_by_str("v0", 0);
                                } else {
                                    self.current_datapath
                                        .set_register_by_str("v0", bytes.len() as u64);
                                }
                            }
                        }
                        self.updates.changed_registers = true;
                        self.updates.changed_memory = true;
                    }
                }
            }
        }
    }

    async fn reset_system(&mut self) {
        self.scanner = Scanner::new();
        self.blocked_on = BlockedOn::Nothing;
        self.initialized = false;
        self.messages = Vec::new();
        self.scope
            .send(DatapathUpdate::System(SystemUpdate::UpdateMessages(
                self.messages.clone(),
            )))
            .await
            .unwrap();
    }

    async fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
        self.scope
            .send(DatapathUpdate::System(SystemUpdate::UpdateMessages(
                self.messages.clone(),
            )))
            .await
            .unwrap();
    }
}
