use crate::agent::EmulationCoreAgent;
use futures::stream::{SplitSink, SplitStream};
use futures::FutureExt;
use futures::SinkExt;
use futures::StreamExt;
use gloo_console::log;
use gloo_console::warn;
use std::cell::RefCell;
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
            if let None = update {
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
}
