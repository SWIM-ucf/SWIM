use crate::agent::datapath_communicator::DatapathCommunicator;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Consoleprops {
    pub communicator: &'static DatapathCommunicator,
    pub messages: Vec<String>,
    pub parsermsg: String,
    pub show_input: UseStateHandle<bool>,
}

#[function_component(Console)]
pub fn console(props: &Consoleprops) -> Html {
    let show_input = props.show_input.clone();
    let input_value = use_state_eq(String::new);

    let on_keyup = {
        let input_value = input_value.clone();
        let communicator = props.communicator;

        use_callback(
            move |event: KeyboardEvent, input_value| {
                let key_code = event.key_code();
                let input_value = input_value.clone();
                // If Enter was pressed parse and send input to emulator core
                if key_code == 13 {
                    communicator.send_input((*input_value).clone());
                    input_value.set("".to_string());
                }
            },
            input_value,
        )
    };

    let on_input = {
        let input_value = input_value.clone();
        use_callback(
            move |event: InputEvent, input_value| {
                let target = event.target();
                let input = target.unwrap().unchecked_into::<HtmlInputElement>();

                input_value.set(input.value());
            },
            input_value,
        )
    };

    html! {
        <div>
            {
                (*props.parsermsg)
                    .split('\n')
                    .map(|line| {
                        html! { <div>{line}</div> }
                    })
                    .collect::<Html>()
            }
            <div>
                {
                    props
                        .messages
                        .iter()
                        .flat_map(|msg| {
                            msg.split('\n').map(|line| {
                                html! { <div>{line}</div> }
                            })
                        })
                        .collect::<Html>()
                }
            </div>
            if *show_input {
                <div class="console-input">
                    <div class="prompt">{">\u{00a0}"}</div> // Prompt followed by a non-breaking space
                    <input type="text" onkeyup={on_keyup} oninput={on_input} value={(*input_value).clone()}/>
                </div>
            }
        </div>
    }
}
