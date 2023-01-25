pub mod emulation_core;
pub mod parser;
pub mod ui;
#[cfg(test)]
pub mod tests;

use stylist::yew::*;
use ui::regview::component::Regview;
use ui::console::component::Console;
use emulation_core::datapath::Datapath;
use emulation_core::mips::datapath::MipsDatapath;
use gloo::console::log;
use monaco::{
    api::TextModel,
    sys::editor::{IEditorMinimapOptions, IStandaloneEditorConstructionOptions},
    yew::CodeEditor,
};
use parser::parser_main::parser;
use std::{cell::RefCell, rc::Rc};
use stylist::css;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew::{html, Html, Properties, Callback};

#[function_component(App)]
fn app() -> Html {
    // This contains the binary representation of "ori $s0, $zero, 12345", which
    // stores 12345 in register $s0.
    let default_code = String::from("ori $s0, $zero, 12345\n");
    let language = String::from("mips");

    // This is the initial text model with default text contents. The
    // use_state_eq hook is created so that the component can be updated
    // when the text model changes.
    let text_model =
        use_state_eq(|| TextModel::create(&default_code, Some(&language), None).unwrap());

    // TODO: Output will be stored in two ways, the first would be the parser's
    // messages via logs and the registers will be stored
    // in a custom-built register viewer.

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
            move |_, text_model| {
                let mut datapath = (*datapath).borrow_mut();

                // parses through the code to assemble the binary
                let (_, assembled) = parser(text_model.get_value());

                // Log initial state of registers in console. (Should all be zero.)
                // NOTE: We are planning on creating a function that makes viewing
                // register data easier. For now, this is manually accessing register data
                // and formatting them all into strings.
                log!(JsValue::from_str(&datapath.registers.to_string()));

                // load the binary into the datapath's memory
                (*datapath)
                    .load_instructions(assembled)
                    .expect("Memory could not be loaded");
                log!(datapath.memory.to_string());
            },
            text_model,
        )
    };

    // This is where the code will get executed.
    let on_execute_clicked = {
        let datapath = Rc::clone(&datapath);
        use_callback(
            move |_, _| {
                let mut datapath = (*datapath).borrow_mut();
                (*datapath).execute_instruction();
                log!(JsValue::from_str(&datapath.registers.to_string()));
            },
            (),
        )
    };

    html! {
        <div>
            <h1>{"Welcome to SWIM"}</h1>
            <button onclick={on_load_clicked}>{ "Assemble" }</button>
            <button onclick={on_execute_clicked}> { "Execute" }</button>
            <Regview />
            <SwimEditor text_model={(*text_model).clone()} />
            <Console />
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
    html! {
        <CodeEditor classes={css!(r#"height: 80vh; width: 80vw;"#)} options={get_options()} model={props.text_model.clone()} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
