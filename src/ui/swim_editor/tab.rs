use std::str::FromStr;
use strum_macros::Display;
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy, Default, Display)]
pub enum TabState {
    #[default]
    Console,
    Datapath,
    HexEditor,
    Editor,
    TextSegment,
    DataSegment,
    StackSegment,
    StackFrameView,
}

impl FromStr for TabState {
    type Err = ();

    fn from_str(tab_name: &str) -> Result<TabState, Self::Err> {
        match tab_name {
            "Console" => Ok(TabState::Console),
            "Datapath" => Ok(TabState::Datapath),
            "HexEditor" => Ok(TabState::HexEditor),
            "Editor" => Ok(TabState::Editor),
            "TextSegment" => Ok(TabState::TextSegment),
            "DataSegment" => Ok(TabState::DataSegment),
            "StackSegment" => Ok(TabState::StackSegment),
            "StackFrameView" => Ok(TabState::StackFrameView),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabProps<T: std::cmp::PartialEq> {
    pub label: String,
    pub text: String,
    pub on_click: Callback<MouseEvent>,
    pub disabled: bool,
    pub active_tab: UseStateHandle<T>,
    pub tab_name: T,
}

#[function_component(Tab)]
pub fn tab<T: PartialEq + 'static>(props: &TabProps<T>) -> Html {
    let active_tab = &props.active_tab;
    let active_class = if **active_tab == props.tab_name {
        "bg-primary-500"
    } else {
        "bg-primary-700"
    };

    html!(
        <button
            class={format!("rounded-t-md py-2 px-4 text-center cursor-pointer duration-300 focus:bg-primary-500 hover:bg-primary-600 active:bg-primary-500 whitespace-nowrap text-ellipsis overflow-hidden  min-w-0 {}", active_class)}
            label={props.label.clone()}
            onclick={props.on_click.clone()}
            disabled={props.disabled}
        >
            {props.text.clone()}
        </button>
    )
}
