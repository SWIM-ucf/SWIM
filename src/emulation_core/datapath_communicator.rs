use futures::stream::{SplitSink, SplitStream};
use futures::StreamExt;
use gloo_console::log;
use yew::UseForceUpdateHandle;
use yew_agent::reactor::ReactorBridge;
use crate::emulation_core::agent::EmulationCoreAgent;

pub struct DatapathCommunicator {
    writer: SplitSink<ReactorBridge<EmulationCoreAgent>, i32>,
    reader: SplitStream<ReactorBridge<EmulationCoreAgent>>,
}

impl DatapathCommunicator {
    pub async fn listen_for_updates(&mut self, update_handle: UseForceUpdateHandle) {
        loop {
            let update = self.reader.next().await;
            log!(format!("Got update {:?}", update));
            update_handle.force_update();
        }
    }
}