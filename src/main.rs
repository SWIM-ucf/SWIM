pub mod emulation_core;
#[cfg(test)]
pub mod tests;
pub mod ui;

use monaco::api::TextModel;
use stylist::yew::*;
use ui::editor::component::Editor;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;

use emulation_core::datapath::Datapath;
use emulation_core::mips::datapath::MipsDatapath;

// NOTE: This should be stored in a child component, but is created in the
// main App component purely for demonstration.

#[styled_component(App)]
fn app() -> Html {
    // This contains the binary representation of "ori $s0, $zero, 12345", which
    // stores 12345 in register $s0.
    let default_code = String::from("00110100000100000011000000111001\n");
    let language = String::from("mips");

    // This is the initial text model with default text contents. The
    // use_state_eq hook is created so that the component can be updated
    // when the text model changes.
    let text_model =
        use_state_eq(|| TextModel::create(&default_code, Some(&language), None).unwrap());

    // Define what happens when you click the run button.
    // NOTE: This is purely for testing and demonstration and should not be
    // exactly replicated in the main project. The datapath should be stored
    // as a property or elsewhere so that state can be maintained. This demonstration
    // will re-create an entirely new datapath on each press of the run button.
    let on_click_run = {
        let text_model = text_model.clone();

        // Return a callback function to execute when the run button is clicked.
        use_callback(
            // This callback function takes the text model as input and performs
            // an instruction based on it.
            move |_, text_model| {
                // Get contents of editor and log them to the console.
                let contents = text_model.get_value();
                console::log_1(&JsValue::from_str(&contents));

                // Convert the string to an array of integers. If invalid text is found, just replace it with 0.
                let instructions: Vec<u32> = contents
                    .split('\n')
                    .map(|text| u32::from_str_radix(text, 2).unwrap_or(0))
                    .collect();

                // Create a new datapath.
                let mut datapath = MipsDatapath::default();

                // Log initial state of registers in console. (Should all be zero.)
                // NOTE: We are planning on creating a function that makes viewing
                // register data easier. For now, this is manually accessing register data
                // and formatting them all into strings.
                let result: Vec<String> = datapath
                    .registers
                    .gpr
                    .iter()
                    .enumerate()
                    .map(|(i, inst)| format!("gpr[{}] = {}", i, inst))
                    .collect();
                let result = result.join("\n");
                console::log_1(&JsValue::from_str(&result));

                // Import one instruction into the beginning datapath memory.
                if !instructions.is_empty() {
                    datapath.memory.store_word(0, instructions[0]).ok();
                }

                // Run one instruction.
                datapath.execute_instruction();

                // Print state of registers after execution.
                // BOOM register 16 contains 12345!
                let result: Vec<String> = datapath
                    .registers
                    .gpr
                    .iter()
                    .enumerate()
                    .map(|(i, inst)| format!("gpr[{}] = {}", i, inst))
                    .collect();
                let result = result.join("\n");
                console::log_1(&wasm_bindgen::JsValue::from_str(&result));
            },
            text_model, // Make text_model a dependency. This will make it so the component updates if text_model changes.
        )
    };

    html! {
        <div>
            <h1>{"Welcome to SWIM"}</h1>
            <button onclick={on_click_run}>{"Test Run Instruction"}</button>
            <Editor text_model={(*text_model).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
