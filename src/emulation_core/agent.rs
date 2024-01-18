use futures::{SinkExt, StreamExt};
use gloo_console::log;
use yew_agent::prelude::*;

#[reactor(EmulationCoreAgent)]
pub async fn emulation_core_agent(mut scope: ReactorScope<i32, i32>) {
    log!("Hello world!");
    scope.send(1).await.unwrap();
    loop {
        let msg = scope.next().await;
        log!("Got message: ", msg);
    }
}
