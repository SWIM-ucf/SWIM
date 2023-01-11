use yew::prelude::*;
use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use monaco::api::CodeEditor;

/*#[derive(Properties, PartialEq)]
pub struct Props {
    pub datapath: dyn Datapath,
}*/

#[function_component(ExecuteButton)]
pub fn execute_button() -> Html {
    html! {
        <button>{"Execute"}</button>
    }
}