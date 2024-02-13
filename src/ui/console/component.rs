use crate::agent::datapath_communicator::DatapathCommunicator;
use web_sys::{KeyboardEvent, InputEvent, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(PartialEq, Properties)]
pub struct Consoleprops {
    pub communicator: &'static DatapathCommunicator,
    pub parsermsg: String,
    pub command: UseStateHandle<String>,
    pub show_input: UseStateHandle<bool>
}

#[function_component(Console)]
pub fn console(props: &Consoleprops) -> Html {
    let show_input = props.show_input.clone();
    let command = props.command.clone();
    let input_value = use_state_eq(String::new);
    let error_msg = use_state_eq(|| "");
    let answered = use_state_eq(|| false);

    let on_keyup = {
        let error_msg = error_msg.clone();
        let input_value = input_value.clone();
        let answered = answered.clone();
        use_callback(move |event: KeyboardEvent, (input_value, error_msg, answered)| {
            // let communicator = props.communicator;
            let key_code = event.key_code();
            let error_msg = error_msg.clone();
            let input_value = input_value.clone();
            let answered = answered.clone();
            // If Enter was pressed parse and send input to emulator core
            if key_code == 13 {
                let input_value = &*input_value;
                log::debug!("Input: {}", (input_value));
                // Parse based on syscall type (int, float, string)
                let val: String = match input_value.parse() {
                    Ok(value) => {
                        value
                    },
                    Err(_err) => {
                        error_msg.set("Invalid input");
                        return
                    }
                };
                answered.set(true);
                log::debug!("{}", val);
                // Send Input command
            }
        }, (input_value, error_msg, answered))
    };

    let on_input = {
        let input_value = input_value.clone();
        let answered = answered.clone();
        use_callback(move |event: InputEvent, (input_value, answered)| {
            // let communicator = props.communicator;
            let target = event.target();
            let input = target.unwrap().unchecked_into::<HtmlInputElement>();

            input_value.set(input.value());
            answered.set(false);
        }, (input_value, answered))
    };

    html! {
        <div>
            {props.parsermsg.clone()}
            <div>
                {*error_msg}
            </div>
            if *answered {
                <div>
                    // { previous_input.iter().collect::<Html>() }
                    {"You answered: "} { (*input_value).clone() }
                </div>
            }
            if *show_input {
                <div class="console-input">
                    <svg viewBox="0 0 330 330" class="console-arrow">
                        <path id="XMLID_222_" d="M250.606,154.389l-150-149.996c-5.857-5.858-15.355-5.858-21.213,0.001 c-5.857,5.858-5.857,15.355,0.001,21.213l139.393,139.39L79.393,304.394c-5.857,5.858-5.857,15.355,0.001,21.213 C82.322,328.536,86.161,330,90,330s7.678-1.464,10.607-4.394l149.999-150.004c2.814-2.813,4.394-6.628,4.394-10.606 C255,161.018,253.42,157.202,250.606,154.389z"/>
                    </svg>
                    <div class="command">{ (*command).clone() } { ": " } </div>
                    <input type="text" onkeyup={on_keyup} oninput={on_input}/>
                </div>
            }
        </div>
    }
}
