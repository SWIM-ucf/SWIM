use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::agent::datapath_reducer::DatapathReducer;
use crate::ui::console::component::Console;
use crate::ui::hex_editor::component::HexEditor;
use crate::ui::visual_datapath::VisualDatapath;
use monaco::api::TextModel;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Footerprops {
    pub communicator: &'static DatapathCommunicator,
    pub datapath_state: UseReducerHandle<DatapathReducer>,
    pub parsermsg: String,
    pub show_input: UseStateHandle<bool>,
    pub memory_text_model: UseStateHandle<TextModel>,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub active_tab: UseStateHandle<FooterTabState>,
    pub on_memory_clicked: Callback<MouseEvent>,
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
                <div class="min-h-48 border-primary-200 border-groove border-2 p-4 max-h-[50%] bg-accent-blue-300 text-primary-200 overflow-y-auto overflow-wrap z-10">
                    <Console datapath_state={props.datapath_state.clone()} communicator={props.communicator} parsermsg={props.parsermsg.clone()} show_input={props.show_input.clone()}/>
                </div>
            } else if **active_tab == FooterTabState::Datapath {
                <VisualDatapath datapath_state={props.datapath_state.clone()} svg_path={svg_path} />
            } else if **active_tab == FooterTabState::HexEditor {
                <div class="min-h-[200px] max-h-[50%] border-primary-200 border-groove border-2 z-10">
                    <HexEditor memory_text_model={props.memory_text_model.clone()} memory_curr_instr={props.memory_curr_instr.clone()} datapath_state={props.datapath_state.clone()}/>
                </div>
            }
            <div class="flex flex-row justify-between w-full">
                <div class="flex flex-row min-w-0">
                    <FooterTab
                        label="console"
                        on_click={change_tab.clone()}
                        disabled={false}
                        active_tab={active_tab.clone()}
                        tab_name={FooterTabState::Console}
                        text="Console"
                    />
                    <FooterTab
                        label="datapath"
                        on_click={change_tab.clone()}
                        disabled={false}
                        active_tab={active_tab.clone()}
                        tab_name={FooterTabState::Datapath}
                        text="Datapath"
                    />
                    <FooterTab
                        label="hex_editor"
                        on_click={change_tab.clone()}
                        disabled={false}
                        active_tab={active_tab.clone()}
                        tab_name={FooterTabState::HexEditor}
                        text="Hex Editor"
                    />
                </div>

                if **active_tab == FooterTabState::Datapath {
                    <div class="min-w-0">
                        <button class="hover:text-primary-100 duration-300 pointer pt-4 min-w-0 text-ellipsis text-nowrap overflow-hidden" onclick={switch_datapath_type}>{switch_datapath_button_label}</button>
                    </div>
                }
                else if **active_tab == FooterTabState::HexEditor {
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
    pub active_tab: UseStateHandle<FooterTabState>,
    pub tab_name: FooterTabState,
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
