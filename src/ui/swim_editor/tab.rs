use yew::prelude::*;

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