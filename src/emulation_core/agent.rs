use yew_agent::prelude::*;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use gloo_console::log;

#[reactor(EmulationCoreAgent)]
pub async fn emulation_core_agent(mut scope: ReactorScope<i32, i32>) {
    log!("Hello world!");
    scope.send(1).await.unwrap();
    loop {
        let _msg = scope.next().await;
    }
}
