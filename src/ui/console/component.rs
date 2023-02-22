use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Consoleprops {
    pub parsermsg: String,
}

#[function_component(Console)]
pub fn console(props: &Consoleprops) -> Html {
    /*let parser_text_output = use_state_eq(String::new);
    let on_error_clicked = {
        let parser_text_output = parser_text_output.clone();
        use_callback(
            move |_, _| {
                parser_text_output.set("Arial".to_string());
            },
            (),
        )
    };*/
    html! {
        <>
            // <div style="width: 80.4%; height: 17vh; border: 1px solid black;"></div>
            <div style="height: 17vh; border: 2px solid black; background-color: #b9cceb; color: #000000;">
                //<button class="button" onclick={on_error_clicked}>{ "Click" }</button>
                <p>{ props.parsermsg.clone() }</p>
                //<p>{ ">" }</p>
            </div>
        </>
    }
}
