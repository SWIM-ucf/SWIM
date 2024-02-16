use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::ui::console::component::Console;
use crate::ui::hex_editor::component::HexEditor;
use crate::ui::visual_datapath::{DatapathSize, VisualDatapath};
use monaco::api::TextModel;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Footerprops {
    pub communicator: &'static DatapathCommunicator,
    pub datapath: MipsDatapath,
    pub parsermsg: String,
    pub show_input: UseStateHandle<bool>,
    pub command: UseStateHandle<String>,
    pub memory_text_model: UseStateHandle<TextModel>,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub active_tab: UseStateHandle<FooterTabState>,
}

#[derive(Default, PartialEq)]
pub enum FooterTabState {
    #[default]
    Console,
    Datapath,
    HexEditor,
}

#[function_component(Footer)]
pub fn footer(props: &Footerprops) -> Html {
    let active_tab = &props.active_tab;
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
                "console" => FooterTabState::Console,
                "datapath" => FooterTabState::Datapath,
                "hex_editor" => FooterTabState::HexEditor,
                _ => FooterTabState::default(),
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

    let svg_path = match *switch_datapath {
        true => "static/datapath_full.svg",
        false => "static/datapath_simple.svg",
    };

    let switch_datapath_button_label = match *switch_datapath {
        true => "Switch to Simple Datapath",
        false => "Switch to Full Datapath",
    };

    html! {
    <>
            // Console buttons
            if **active_tab == FooterTabState::Console {
                <div class="console-wrapper">
                    <Console communicator={props.communicator} parsermsg={props.parsermsg.clone()} show_input={props.show_input.clone()} command={props.command.clone()}/>
                </div>
            } else if **active_tab == FooterTabState::Datapath {
                <div class="datapath-wrapper">
                    <VisualDatapath datapath={props.datapath.clone()} svg_path={svg_path} size={datapath_size} />
                </div>
            } else if **active_tab == FooterTabState::HexEditor {
                <div class="hex-wrapper">
                    <HexEditor memory_text_model={props.memory_text_model.clone()} instruction_num={props.memory_curr_instr.clone()}/>
                </div>
            }
            <div class="button-bar">
                <div class="tabs">
                    if **active_tab == FooterTabState::Console {
                        <button class={classes!("bottom-tab", "pressed")} label="console" onclick={change_tab.clone()}>{"Console"}</button>
                    } else {
                        <button class="bottom-tab" label="console" onclick={change_tab.clone()}>{"Console"}</button>
                    }

                    if **active_tab == FooterTabState::Datapath {
                        <button class={classes!("bottom-tab", "pressed")} label="datapath" onclick={change_tab.clone()}>{"Datapath"}</button>
                    } else {
                        <button class="bottom-tab" label="datapath" onclick={change_tab.clone()}>{"Datapath"}</button>
                    }

                    if **active_tab == FooterTabState::HexEditor {
                        <button class={classes!("bottom-tab", "pressed")} label="hex_editor" onclick={change_tab.clone()}>{"Hex Editor"}</button>
                    } else {
                        <button class="bottom-tab" label="hex_editor" onclick={change_tab.clone()}>{"Hex Editor"}</button>
                    }
                </div>

                if **active_tab == FooterTabState::Datapath {
                    <div class="buttons">
                        <button class={ classes!("bg-red-500", "button") } onclick={toggle_zoom}>{"Toggle Zoom"}</button>
                        <button class="button" onclick={switch_datapath_type}>{switch_datapath_button_label}</button>
                    </div>
                }
            </div>
        </>
    }
}
