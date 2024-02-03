//use crate::parser::parser_structs_and_enums::instruction_tokenization::ProgramInfo;
//use monaco::api::TextModel;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;


use monaco::api::TextModel;

use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::ui::visual_datapath::{DatapathSize, VisualDatapath};
use crate::ui::hex_editor::component::HexEditor;

#[derive(PartialEq, Properties)]
pub struct Consoleprops {
    pub datapath: MipsDatapath,
    pub parsermsg: String,
    pub memory_text_model: Rc<RefCell<TextModel>>,
    pub memory_curr_line: UseStateHandle<f64>
}

#[derive(Default, PartialEq)]
enum TabState {
    #[default]
    Console,
    Datapath,
    HexEditor
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
                "hex_editor" => TabState::HexEditor,
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
            if *active_tab == TabState::Console {
                <pre class="console">
                    { props.parsermsg.clone() }
                </pre>
            } else if *active_tab == TabState::Datapath {
                <div class="datapath-wrapper">
                    <VisualDatapath datapath={props.datapath.clone()} svg_path={svg_path} size={datapath_size} />
                </div>
            } else if *active_tab == TabState::HexEditor {
                <div class="hex-wrapper">
                    <HexEditor memory_text_model={&props.memory_text_model} curr_line={props.memory_curr_line.clone()}/>
                </div>
            }
            <div class="button-bar">
                <div class="tabs">
                    if *active_tab == TabState::Console {
                        <button class={classes!("bottom-tab", "pressed")} label="console" onclick={change_tab.clone()}>{"Console"}</button>
                    } else {
                        <button class="bottom-tab" label="console" onclick={change_tab.clone()}>{"Console"}</button>
                    }

                    if *active_tab == TabState::Datapath {
                        <button class={classes!("bottom-tab", "pressed")} label="datapath" onclick={change_tab.clone()}>{"Datapath"}</button>
                    } else {
                        <button class="bottom-tab" label="datapath" onclick={change_tab.clone()}>{"Datapath"}</button>
                    }

                    if *active_tab == TabState::HexEditor {
                        <button class={classes!("bottom-tab", "pressed")} label="hex_editor" onclick={change_tab.clone()}>{"Hex Editor"}</button>
                    } else {
                        <button class="bottom-tab" label="hex_editor" onclick={change_tab.clone()}>{"Hex Editor"}</button>
                    }
                </div>

                if *active_tab == TabState::Datapath {
                    <div class="buttons">
                        <button class={ classes!("bg-red-500", "button") } onclick={toggle_zoom}>{"Toggle Zoom"}</button>
                        <button class="button" onclick={switch_datapath_type}>{switch_datapath_button_label}</button>
                    </div>
                }
            </div>
        </>
    }
}
