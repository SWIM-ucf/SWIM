//use crate::parser::parser_structs_and_enums::instruction_tokenization::ProgramInfo;
//use monaco::api::TextModel;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::ui::visual_datapath::{DatapathSize, VisualDatapath};

#[derive(PartialEq, Properties)]
pub struct Consoleprops {
    pub datapath: MipsDatapath,
    pub parsermsg: String,
    pub memorymsg: String,
}

#[derive(Default, PartialEq)]
enum TabState {
    #[default]
    Console,
    Datapath,
    Memory,
}

#[function_component(Console)]
pub fn console(props: &Consoleprops) -> Html {
    let active_tab = use_state_eq(TabState::default);
    let zoom_datapath = use_bool_toggle(false);
    let switch_datapath = use_bool_toggle(false);
    let change_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            let tab_name = target
                .get_attribute("label")
                .unwrap_or(String::from("console"));

            let new_tab = match tab_name.as_str() {
                "console" => TabState::Console,
                "datapath" => TabState::Datapath,
                "memory" => TabState::Memory,
                _ => TabState::default(),
            };

            active_tab.set(new_tab);
        })
    };

    let toggle_zoom = {
        let zoom_datapath = zoom_datapath.clone();

        Callback::from(move |_| {
            zoom_datapath.toggle();
        })
    };

    let datapath_size = match *zoom_datapath {
        true => DatapathSize::Big,
        false => DatapathSize::Small,
    };

    let switch_datapath_type = {
        let switch_datapath = switch_datapath.clone();

        Callback::from(move |_| {
            switch_datapath.toggle();
        })
    };

    let datapath_type = match *switch_datapath {
        true => "static/datapath_full.svg",
        false => "static/datapath_simple.svg",
    };

    html! {
    <>
            // Console buttons
            if *active_tab == TabState::Console {
                <pre class="console">
                    { props.parsermsg.clone() }
                </pre>
            } else if *active_tab == TabState::Datapath {
                <div class="datapath-wrapper">
                    <VisualDatapath datapath={props.datapath.clone()} svg_path={datapath_type} size={datapath_size} />
                </div>
            } else {
                <div class="console">
                    <pre class = "memory-view">
                        {props.datapath.memory.generate_formatted_hex() }
                    </pre>
                </div>
            }
            <div class="tabs">
                <button class="tab" label="console" onclick={change_tab.clone()}>{"Console"}</button>
                <button class="tab" label="datapath" onclick={change_tab.clone()}>{"Datapath"}</button>
                <button class="tab" label="memory" onclick={change_tab.clone()}>{"Memory"}</button>

                if *active_tab == TabState::Datapath {
                    <button onclick={toggle_zoom}>{"Toggle Zoom"}</button>
                    <button onclick={switch_datapath_type}>{"Switch Datapath View"}</button>

                }
            </div>
        </>
    }
}
