//! The agent responsible for running the emulator core on the worker thread and communication functionalities.

use futures::{SinkExt, StreamExt};
use gloo_console::log;
use yew_agent::prelude::*;

pub mod datapath_communicator;

/// The main logic for the emulation core agent. All code within this function runs on a worker thread as opposed to
/// the UI thread.
#[reactor(EmulationCoreAgent)]
pub async fn emulation_core_agent(mut scope: ReactorScope<i32, i32>) {
    log!("Hello world!");
    scope.send(1).await.unwrap();
    loop {
        let msg = scope.next().await;
        log!("Got message: ", msg);
    }
}
