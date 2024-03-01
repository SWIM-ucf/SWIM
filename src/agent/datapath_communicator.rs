use crate::agent::datapath_reducer::DatapathReducer;
use crate::agent::messages::Command;
use crate::agent::EmulationCoreAgent;
use crate::emulation_core::architectures::AvailableDatapaths;
use futures::stream::{SplitSink, SplitStream};
use futures::FutureExt;
use futures::SinkExt;
use futures::StreamExt;
use gloo_console::log;
use gloo_console::warn;
use std::cell::RefCell;
use yew::UseReducerDispatcher;
use yew_agent::reactor::ReactorBridge;

/// This struct provides an abstraction over all communication with the worker thread. Any commands to the worker
/// thread should be sent by calling a function on this struct.
///
/// The DatapathCommunicator will also handle receiving information about the state of the emulation core and maintain
/// internal state that can be displayed by the UI.
pub struct DatapathCommunicator {
    writer: RefCell<SplitSink<ReactorBridge<EmulationCoreAgent>, Command>>,
    reader: RefCell<SplitStream<ReactorBridge<EmulationCoreAgent>>>,
}

// Check references for equality by memory address.
impl PartialEq for &DatapathCommunicator {
    fn eq(&self, other: &Self) -> bool {
        let self_ptr: *const DatapathCommunicator = *self;
        let other_ptr: *const DatapathCommunicator = *other;
        self_ptr == other_ptr
    }
}

impl DatapathCommunicator {
    // General operational functions

    /// Initialize the DatapathCommunicator using a bridge.
    pub fn new(bridge: ReactorBridge<EmulationCoreAgent>) -> DatapathCommunicator {
        let (write, read) = bridge.split();
        DatapathCommunicator {
            writer: RefCell::new(write),
            reader: RefCell::new(read),
        }
    }

    /// Listen for updates from the worker thread and update internal state accordingly. This function should be called
    /// from the main app component. After updating internal state, the component this was called from will be force
    /// updated.
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn listen_for_updates(
        &self,
        dispatcher_handle: UseReducerDispatcher<DatapathReducer>,
    ) {
        let mut reader = match self.reader.try_borrow_mut() {
            Ok(reader) => reader,
            Err(_) => {
                warn!("Warning: Attempted to listen for updates multiple times");
                return;
            }
        };

        loop {
            log!("Waiting...");
            let update = reader.next().await;
            log!(format!("Got update {:?}", update));
            match update {
                None => return,
                Some(update) => dispatcher_handle.dispatch(update),
            }
        }
    }

    /// Sends a test message to the worker thread.
    fn send_message(&self, command: Command) {
        let mut writer = self.writer.borrow_mut();
        writer
            .send(command)
            // The logic for sending a message is synchronous but the API for writing to a SplitSink is asynchronous,
            // so we attempt to resolve the future immediately so we can expose a synchronous API for sending commands.
            // If the future doesn't return immediately, there's serious logic changes that need to happen so we just
            // log an error message and panic.
            .now_or_never()
            .expect("Send function did not immediately return, async logic needed.")
            .expect("Sending test message error")
    }

    // Wrapper functions for commands

    /// Sets the current emulation core to the provided architecture.
    pub fn set_core(&self, _architecture: AvailableDatapaths) {
        todo!()
    }

    /// Resets and loads the parsed/assembled instructions provided into the current emulator core.
    pub fn initialize(&self, initial_pc: usize, instructions: Vec<u32>) {
        self.send_message(Command::Initialize(initial_pc, instructions));
    }

    /// Sets the execution speed of the emulator core to the provided speed in hz. If set to zero, the emulator core
    /// will execute as fast as possible.
    pub fn set_execute_speed(&self, speed: u32) {
        self.send_message(Command::SetExecuteSpeed(speed));
    }

    /// Sets the register with the provided name to the provided value.
    pub fn set_register(&self, register: String, data: u64) {
        self.send_message(Command::SetRegister(register, data));
    }

    // Sets the FP register with the provided name to the provided value.
    pub fn set_fp_register(&self, register: String, data: u64) {
        self.send_message(Command::SetFPRegister(register, data));
    }

    /// Copies the contents of `data` to the emulator core's memory at `ptr`. Copies until either the end of `data` or
    /// the end of the emulaot core's memory.
    pub fn set_memory(&self, ptr: u64, data: u32) {
        self.send_message(Command::SetMemory(ptr, data));
    }

    /// Executes the emulator core at the current set speed.
    pub fn execute(&self) {
        self.send_message(Command::Execute);
    }

    /// Executes a single instruction on the emulator core and pauses.
    pub fn execute_instruction(&self) {
        self.send_message(Command::ExecuteInstruction);
    }

    /// Executes a single stage on the emulator core and pauses.
    pub fn execute_stage(&self) {
        self.send_message(Command::ExecuteStage);
    }

    /// Pauses the core. Does nothing if the emulator core is already paused.
    pub fn pause_core(&self) {
        self.send_message(Command::Pause);
    }

    /// Resets the current core to its default state.
    pub fn reset(&self) {
        self.send_message(Command::Reset);
    }

    pub fn get_accepting_input(&self) -> bool {
        todo!()
    }
}
