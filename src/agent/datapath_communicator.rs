use crate::agent::EmulationCoreAgent;
use crate::emulation_core::architectures::AvailableDatapaths;
use futures::stream::{SplitSink, SplitStream};
use futures::FutureExt;
use futures::SinkExt;
use futures::StreamExt;
use gloo_console::log;
use gloo_console::warn;
use std::cell::RefCell;
use std::collections::HashMap;
use yew::UseForceUpdateHandle;
use yew_agent::reactor::ReactorBridge;

/// This struct provides an abstraction over all communication with the worker thread. Any commands to the worker
/// thread should be sent by calling a function on this struct.
///
/// The DatapathCommunicator will also handle receiving information about the state of the emulation core and maintain
/// internal state that can be displayed by the UI.
pub struct DatapathCommunicator {
    writer: RefCell<SplitSink<ReactorBridge<EmulationCoreAgent>, i32>>,
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
    pub async fn listen_for_updates(&self, update_handle: UseForceUpdateHandle) {
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
            if update.is_none() {
                return;
            }
            update_handle.force_update();
        }
    }

    /// Sends a test message to the worker thread.
    pub fn send_test_message(&self) {
        let mut writer = self.writer.borrow_mut();
        writer
            .send(1)
            // The
            .now_or_never()
            .expect("Send function did not immediately return, async logic needed.")
            .expect("Sending test message error")
    }

    // Wrapper functions for commands

    pub fn set_core(&self, _architecture: AvailableDatapaths) {
        todo!()
    }

    pub fn load_instructions(&self, _instructions: &[u8]) {
        todo!()
    }

    pub fn set_execute_speed(&self, _speed: u32) {
        todo!()
    }

    pub fn set_register(&self, _register: &str, _data: &str) {
        todo!()
    }

    pub fn set_memory(&self, _ptr: usize, _data: &[u8]) {
        todo!()
    }

    pub fn execute(&self) {
        todo!()
    }

    pub fn execute_instruction(&self) {
        todo!()
    }

    pub fn execute_stage(&self) {
        todo!()
    }

    pub fn pause_core(&self) {}

    // Getters for internal state

    pub fn get_registers(&self) -> HashMap<String, String> {
        todo!()
    }

    pub fn get_memory(&self) -> Vec<u8> {
        todo!()
    }

    pub fn get_current_stage(&self) -> String {
        todo!()
    }

    pub fn get_current_instruction(&self) -> usize {
        todo!()
    }
}
