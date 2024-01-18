use crate::emulation_core::agent::EmulationCoreAgent;
use futures::stream::{SplitSink, SplitStream};
use futures::FutureExt;
use futures::SinkExt;
use futures::StreamExt;
use gloo_console::log;
use gloo_console::warn;
use std::cell::RefCell;
use yew::UseForceUpdateHandle;
use yew_agent::reactor::ReactorBridge;

pub struct DatapathCommunicator {
    writer: RefCell<SplitSink<ReactorBridge<EmulationCoreAgent>, i32>>,
    reader: RefCell<SplitStream<ReactorBridge<EmulationCoreAgent>>>,
}

impl DatapathCommunicator {
    pub fn new(bridge: ReactorBridge<EmulationCoreAgent>) -> DatapathCommunicator {
        let (write, read) = bridge.split();
        DatapathCommunicator {
            writer: RefCell::new(write),
            reader: RefCell::new(read),
        }
    }

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
            let update = reader.next().await;
            log!(format!("Got update {:?}", update));
            update_handle.force_update();
        }
    }

    pub fn send_test_message(&self) {
        let mut writer = self.writer.borrow_mut();
        writer
            .send(1)
            .now_or_never()
            .expect("Send function did not immediately return, async logic needed.")
            .expect("Sending test message error")
    }
}
