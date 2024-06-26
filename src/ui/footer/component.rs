use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::agent::datapath_reducer::DatapathReducer;
use crate::emulation_core::architectures::AvailableDatapaths::{MIPS, RISCV};
use crate::emulation_core::mips::memory::Memory;
use crate::ui::console::component::Console;
use crate::ui::hex_editor::component::HexEditor;
use crate::ui::swim_editor::tab::TabState;
use crate::ui::visual_datapath::VisualDatapath;
use monaco::api::TextModel;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

// ** Footer Component ** //
// The footer component is a container for the console, datapath, and hex editor components
// It holds the tabs for each of these components and allows the user to switch between them

#[derive(PartialEq, Properties)]
pub struct Footerprops {
    pub communicator: &'static DatapathCommunicator,
    pub datapath_state: UseReducerHandle<DatapathReducer>,
    pub parsermsg: String,
    pub show_input: UseStateHandle<bool>,
    pub memory_text_model: UseStateHandle<TextModel>,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub active_tab: UseStateHandle<TabState>,
    pub on_memory_clicked: Callback<MouseEvent>,
    pub memory: Memory,
    pub pc: u64,
}

#[function_component(Footer)]
pub fn footer(props: &Footerprops) -> Html {
    let active_tab = &props.active_tab;
    let switch_datapath = use_bool_toggle(false);
    let change_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            let tab_name = target
                .get_attribute("label")
                .unwrap_or(String::from("console"));

            let new_tab = TabState::from_str(&tab_name).unwrap();
            active_tab.set(new_tab);
        })
    };

    let switch_datapath_type = {
        let switch_datapath = switch_datapath.clone();

        Callback::from(move |_| {
            switch_datapath.toggle();
        })
    };
    let svg_path = match props.datapath_state.current_architecture {
        MIPS => match *switch_datapath {
            true => "static/datapath_full.svg",
            false => "static/datapath_simple.svg",
        },
        RISCV => "static/datapath_riscv.svg",
    };

    let switch_datapath_button_label = match *switch_datapath {
        true => "Switch to Simple Datapath",
        false => "Switch to Full Datapath",
    };

    html! {
    <>
            // Console buttons
            if **active_tab == TabState::Console {
                <div class="h-48 border-primary-200 border-groove border-2 p-4 bg-accent-blue-300 text-primary-200 overflow-y-auto overflow-wrap z-10">
                    <Console
                        messages={props.datapath_state.messages.clone()}
                        communicator={props.communicator}
                        parsermsg={props.parsermsg.clone()}
                        show_input={props.show_input.clone()}
                    />
                </div>
            } else if **active_tab == TabState::Datapath {
                <VisualDatapath datapath_state={props.datapath_state.clone()} svg_path={svg_path} />
            } else if **active_tab == TabState::HexEditor {
                <div class="flex h-48 border-primary-200 border-groove border-2 z-10">
                    <HexEditor
                        memory_text_model={props.memory_text_model.clone()}
                        memory_curr_instr={props.memory_curr_instr.clone()}
                        memory={props.memory.clone()}
                        pc={props.pc}
                        initialized={props.datapath_state.initialized}
                        executing={props.datapath_state.executing}
                    />
                </div>
            }
            <div class="flex flex-row justify-between w-full">
                <div class="flex flex-row min-w-0">
                    <FooterTab
                        label={TabState::Console.to_string()}
                        on_click={change_tab.clone()}
                        disabled={false}
                        active_tab={active_tab.clone()}
                        tab_name={TabState::Console}
                        text="Console"
                    />
                    <FooterTab
                        label={TabState::Datapath.to_string()}
                        on_click={change_tab.clone()}
                        disabled={false}
                        active_tab={active_tab.clone()}
                        tab_name={TabState::Datapath}
                        text="Datapath"
                    />
                    <FooterTab
                        label={TabState::HexEditor.to_string()}
                        on_click={change_tab.clone()}
                        disabled={false}
                        active_tab={active_tab.clone()}
                        tab_name={TabState::HexEditor}
                        text="Hex Editor"
                    />
                </div>

                if **active_tab == TabState::Datapath && props.datapath_state.current_architecture == MIPS {
                    <div class="min-w-0">
                        <button class="hover:text-primary-100 duration-300 pointer pt-4 min-w-0 text-ellipsis text-nowrap overflow-hidden" onclick={switch_datapath_type}>{switch_datapath_button_label}</button>
                    </div>
                }
                else if **active_tab == TabState::HexEditor {
                    <div class="min-w-0">
                        <button class="disabled:hidden hover:text-primary-100 duration-300 pointer pt-4 min-w-0 text-ellipsis text-nowrap overflow-hidden" onclick={props.on_memory_clicked.clone()} disabled={!props.datapath_state.initialized}>{"Update Memory"}</button>
                    </div>
                }
            </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct FooterTabProps {
    pub label: String,
    pub text: String,
    pub on_click: Callback<MouseEvent>,
    pub disabled: bool,
    pub active_tab: UseStateHandle<TabState>,
    pub tab_name: TabState,
}

#[function_component(FooterTab)]
pub fn footer_tab(props: &FooterTabProps) -> Html {
    let active_tab = &props.active_tab;
    html!(
        if **active_tab == props.tab_name {
            <button class="text-primary-100 font-bold hover:text-primary-100 pt-3 px-8 pointer border-t-4 border-solid border-primary-100 w-40 min-w-0 overflow-hidden text-ellipsis text-nowrap" label={props.label.clone()} onclick={props.on_click.clone()} disabled={props.disabled}>{props.text.clone()}</button>
        } else {
            <button class="hover:text-primary-100 pt-3 px-8 pointer border-top-4 border-solid border-transparent w-40 min-w-0 text-ellipsis overflow-hidden text-nowrap" label={props.label.clone()} onclick={props.on_click.clone()} disabled={props.disabled}>{props.text.clone()}</button>
        }
    )
}
