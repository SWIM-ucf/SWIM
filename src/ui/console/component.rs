use monaco::api::TextModel;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::{ProgramInfo};
use yew_hooks::prelude::*;

use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::ui::visual_datapath::{DatapathSize, VisualDatapath};
use crate::parser::parser_assembler_main::parser;
use std::{cell::RefCell, rc::Rc};

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
    let code = String::from("ori $s0, $zero, 12345\n");
    let language = String::from("mips");
    let text_model = use_state_eq(|| {
        Rc::new(RefCell::new(
            TextModel::create(&code, Some(&language), None).unwrap(),
        ))
    });
    let text_model = (*text_model).borrow_mut();
    let (tmi, _) = parser(text_model.get_value());
    let instructions = tmi.instructions;
    let data = tmi.data;
    let address_to_line_number = tmi.address_to_line_number;
    let monaco_line_info = tmi.monaco_line_info;
    let updated_monaco_string = tmi.updated_monaco_string;

    let active_tab = use_state_eq(TabState::default);
    let zoom_datapath = use_bool_toggle(false);
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

    html! {
        <>
            if *active_tab == TabState::Console {
                <div class="console">
                    { hello_string(&ProgramInfo { instructions: (instructions), data: (data),
                        address_to_line_number: (address_to_line_number), monaco_line_info: (monaco_line_info), 
                        updated_monaco_string: (updated_monaco_string) })}
                </div>
            } else if *active_tab == TabState::Datapath {
                <div class="datapath-wrapper">
                    <VisualDatapath datapath={props.datapath.clone()} svg_path={"static/datapath.svg"} size={datapath_size} />
                </div>
            } else {
                <div class="console">
                    { props.datapath.memory.to_string() }
                </div>
            }

            // Console buttons
            <div class="tabs">
                <button class="tab" label="console" onclick={change_tab.clone()}>{"Console"}</button>
                <button class="tab" label="datapath" onclick={change_tab.clone()}>{"Datapath"}</button>
                <button class="tab" label="memory" onclick={change_tab.clone()}>{"Memory"}</button>

                if *active_tab == TabState::Datapath {
                    <button onclick={toggle_zoom}>{"Toggle Zoom"}</button>
                }
            </div>
        </>
    }
}

fn hello_string(_x: &ProgramInfo) -> String {
    return "hello world".to_string();
}