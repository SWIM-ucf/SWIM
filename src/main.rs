pub mod emulation_core;
#[cfg(test)]
pub mod tests;

<<<<<<< Updated upstream
use emulation_core::datapath::Datapath;
use emulation_core::mips::datapath::MipsDatapath;
use monaco::{
    api::{TextModel},
    sys::editor::{IEditorMinimapOptions, IStandaloneEditorConstructionOptions},
    yew::{CodeEditor, CodeEditorLink}
};
use stylist::css;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;

=======
use std::{rc::Rc, cell::RefCell};
use emulation_core::datapath::Datapath;
use emulation_core::mips::datapath::MipsDatapath;
use gloo::console::log;
use parser::parser_main::parser;
use monaco::{
    api::{TextModel},
    sys::editor::{IEditorMinimapOptions, IStandaloneEditorConstructionOptions},
    yew::CodeEditor
};
use stylist::css;
use wasm_bindgen::JsValue;
use yew::prelude::*;


>>>>>>> Stashed changes
#[function_component(App)]
fn app() -> Html {
    
    // This contains the binary representation of "ori $s0, $zero, 12345", which
    // stores 12345 in register $s0.
<<<<<<< Updated upstream
    let default_code = String::from("00110100000100000011000000111001\n");
=======
    let default_code = String::from("ori $s0, $zero, 12345\n");
>>>>>>> Stashed changes
    let language = String::from("mips");
    
    // This is the initial text model with default text contents. The
    // use_state_eq hook is created so that the component can be updated
    // when the text model changes.
    let text_model = 
        use_state_eq(|| TextModel::create(&default_code, Some(&language), None).unwrap());
<<<<<<< Updated upstream
   
    // This is where the code will be stored. 
    let code = use_state_eq(|| String ::from(default_code));
=======
>>>>>>> Stashed changes
    
    // TODO: Output will be stored in two ways, the first would be the parser's 
    // messages via logs and the registers will be stored 
    // in a custom-built register viewer.

<<<<<<< Updated upstream
    // Since we want the Datapath to be independent from all the events within the app,
    // we will create it when the app loads. This is also done since the scope will be
    // open across all events involved with it.
    let mut datapath = MipsDatapath::default();

    // This is where the CodeEditor's TextModel will be taken to be turned into code
    // for the emulation core/parser to process with a button.
    let on_move_clicked = {
        let text_model = text_model.clone();
        let code = code.clone();
        use_callback(
            move |_, text_model|{
                code.set(text_model.get_value());
                console::log_1(&JsValue::from_str(&code));
            },
            text_model,
        )
    };

    // This is where we take the code to transform it to binary to load it into memory
    let on_load_clicked = {
        let code = code.clone();
        use_callback(
            move |_, code|{
                
                // Convert the string to an array of integers. If invalid text is found, just replace it with 0.
                let instructions: Vec<u32> = code
                    .split('\n')
                    .map(|text| u32::from_str_radix(text, 2).unwrap_or(0))
                    .collect();
                
=======
    // Since we want the Datapath to be independent from all the 
    // events within the app, we will create it when the app loads. This is also done 
    // since the scope will be open across all events involved with it. To achieve this,
    // we use interior mutability to have the reference to the Datapath immutable, but
    // the ability to access and change its contents be mutable.
    let datapath = Rc::new(RefCell::new(MipsDatapath::default()));

    // This is where we take the code and run it through the emulation core
    let on_load_clicked = {
        let text_model = text_model.clone();
        let datapath = Rc::clone(&datapath);
        use_callback(
            move |_, text_model|{
                let mut datapath = (*datapath).borrow_mut();

                // parses through the code to assemble the binary
                let (_, assembled) = parser(text_model.get_value());

>>>>>>> Stashed changes
                // Log initial state of registers in console. (Should all be zero.)
                // NOTE: We are planning on creating a function that makes viewing
                // register data easier. For now, this is manually accessing register data
                // and formatting them all into strings.
<<<<<<< Updated upstream
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
            },code
=======
                log!(JsValue::from_str(&datapath.registers.to_string()));

                // load the binary into the datapath's memory
                (*datapath).load_instructions(assembled);
                log!((*datapath).memory.to_string());
            }, text_model
        )
    };

    // This is where the code will get executed.
    let on_execute_clicked = {
        let datapath = Rc::clone(&datapath);
        use_callback(
            move |_, _|{
                let mut datapath = (*datapath).borrow_mut();
                (*datapath).execute_instruction();
                log!(JsValue::from_str(&datapath.registers.to_string()));
            }, ()
>>>>>>> Stashed changes
        )
    };
    
    
    html! {
        <div>
            <h1>{"Welcome to SWIM"}</h1>
<<<<<<< Updated upstream
            <button onclick={on_move_clicked}>{ "Text2Code" }</button>
            <button onclick={on_load_clicked}>{ "Code2Mem" }</button>
=======
            <button onclick={on_load_clicked}>{ "Assemble" }</button>
            <button onclick={on_execute_clicked}> { "Execute" }</button>
>>>>>>> Stashed changes
            <SwimEditor text_model={(*text_model).clone()} />
        </div>
    }
}

/**********************  Editor Component **********************/

#[derive(PartialEq, Properties)]
pub struct SwimEditorProps {
    text_model: TextModel,
}

fn get_options() -> IStandaloneEditorConstructionOptions {
    let options = IStandaloneEditorConstructionOptions::default();
    options.set_theme("vs-dark".into());
    options.set_language("mips".into());
    options.set_scroll_beyond_last_line(false.into());
    options.set_automatic_layout(true.into());

    let minimap = IEditorMinimapOptions::default();
    minimap.set_enabled(false.into());
    options.set_minimap(Some(&minimap));

    options
}

#[function_component]
pub fn SwimEditor(props: &SwimEditorProps) -> Html {
<<<<<<< Updated upstream
    let SwimEditorProps {
        text_model,
    } = props;
    let link: UseStateHandle<CodeEditorLink> = use_state(CodeEditorLink::default);
    html!{
        <CodeEditor classes={css!(r#"height: 80vh; width: 80vw;"#)} options={get_options()} link={(*link).clone()} model={text_model.clone()} />
=======
    html!{
        <CodeEditor classes={css!(r#"height: 80vh; width: 80vw;"#)} options={get_options()} model={props.text_model.clone()} />
>>>>>>> Stashed changes
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
