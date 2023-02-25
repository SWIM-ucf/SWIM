pub mod emulation_core;
pub mod parser;
#[cfg(test)]
pub mod tests;
pub mod ui;

use emulation_core::datapath::Datapath;
use emulation_core::mips::datapath::MipsDatapath;
use gloo::{console::log, file::FileList};
use js_sys::{Array, Object};
use monaco::{
    api::TextModel,
    sys::editor::{
        IEditorMinimapOptions, IEditorScrollbarOptions, IStandaloneEditorConstructionOptions,
    },
    yew::{CodeEditor, CodeEditorLink},
};
use parser::parser_main::parser;
use std::{cell::RefCell, rc::Rc};
use stylist::css;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
//use stylist::yew::*;
use ui::console::component::Console;
use ui::regview::component::Regview;
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew::{html, Html, Properties};

#[function_component(App)]
fn app() -> Html {
    // This contains the binary representation of "ori $s0, $zero, 12345", which
    // stores 12345 in register $s0.
    let code = String::from("ori $s0, $zero, 12345\n");
    let language = String::from("mips");

    // This is the initial text model with default text contents. The
    // use_state_eq hook is created so that the component can be updated
    // when the text model changes.
    let text_model = use_state_eq(|| {
        Rc::new(RefCell::new(
            TextModel::create(&code, Some(&language), None).unwrap(),
        ))
    });

    // Link to the Yew Editor Component, if not used by the end of the project remove it.
    let codelink = CodeEditorLink::default();

    // Setup the array that would store decorations applied to the
    // text model and initialize the options for it.
    let delta_decor = monaco::sys::editor::IModelDecorationOptions::default();
    let decor_array = use_state_eq(|| js_sys::Array::new());
    log!("This is the array when the app loads");
    log!((*decor_array).at(0));

    // TODO: Output will be stored in two ways, the first would be the parser's
    // messages via logs and the registers will be stored
    // in a custom-built register viewer.
    let parser_text_output = use_state_eq(String::new);

    // Since we want the Datapath to be independent from all the
    // events within the app, we will create it when the app loads. This is also done
    // since the scope will be open across all events involved with it. To achieve this,
    // we use interior mutability to have the reference to the Datapath immutable, but
    // the ability to access and change its contents be mutable.
    let datapath = use_state_eq(|| Rc::new(RefCell::new(MipsDatapath::default())));

    // This is where we take the code and run it through the emulation core
    let on_assemble_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();
        use_callback(
            move |_, text_model| {
                let mut datapath = (*datapath).borrow_mut();
                let text_model = (*text_model).borrow_mut();

                // parses through the code to assemble the binary
                let (_, assembled) = parser(text_model.get_value());
                // log!(JsValue::from_str(&datapath.registers.to_string()));
                // Load the binary into the datapath's memory
                (*datapath)
                    .load_instructions(assembled)
                    .expect("Memory could not be loaded");
                //log!(datapath.memory.to_string());
                trigger.force_update();
            },
            text_model,
        )
    };

    // This is where the code will get executed. If you execute further
    // than when the code ends, the program crashes.
    let on_execute_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();

        let decor_array = decor_array.clone();
        log!("This is the array before the push");
        log!((*decor_array).at(0));

        use_callback(
            move |_, _| {
                let mut datapath = (*datapath).borrow_mut();

                let text_model = (*text_model).borrow_mut();
                let curr_model = text_model.as_ref();
                // Setting up the options/parameters which
                // will highlight the executed line.
                // Currently, just highlights the first line as
                // a proof of concept.
                let curr_range = monaco::sys::Range::new(1.0, 0.0, 1.0, 0.0);
                let range_js = curr_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                delta_decor.set_is_whole_line(true.into());
                delta_decor.set_inline_class_name("myInlineDecoration".into());

                // element to be stored in the Decoration array (keep this)
                let highlight_line: monaco::sys::editor::IModelDeltaDecoration =
                    Object::new().unchecked_into();
                highlight_line.set_options(&delta_decor);
                highlight_line.set_range(&monaco::sys::IRange::from(range_js));
                let highlight_js = highlight_line
                    .dyn_into::<JsValue>()
                    .expect("Highlight is not found.");
                decor_array.push(&highlight_js); // create a function to do this
                (*curr_model).delta_decorations(&js_sys::Array::new(), &decor_array, None);
                (*datapath).execute_instruction();
                log!("This is the array after the push");
                log!((*decor_array).at(0));
                log!((*curr_model).delta_decorations(&js_sys::Array::new(), &decor_array, None));
                // log!(JsValue::from_str(&datapath.registers.to_string()));
                trigger.force_update();
            },
            (),
        )
    };

    // This is how we will reset the datapath. This is the only method to "halt"
    // programs since if the user continues to execute, the whole application will
    // crash.
    let on_reset_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();
        use_callback(
            move |_, _| {
                let mut datapath = (*datapath).borrow_mut();
                let text_model = (*text_model).borrow_mut();
                let curr_model = text_model.as_ref();
                (*curr_model).delta_decorations(&decor_array, &js_sys::Array::new(), None);
                (*datapath).reset();
                log!("The handle should still be there, no highlight");
                log!((*decor_array).at(0));
                // log!(JsValue::from_str(&datapath.registers.to_string()));
                trigger.force_update();
            },
            (),
        )
    };

    // This is where the parser will output its error messages to the user.
    // Currently, it is tied to a button with placeholder text. The goal is to have
    // this action take place when the Text Model changes and output the messages provided
    // by the parser.
    let on_error_clicked = {
        let parser_text_output = parser_text_output.clone();
        use_callback(
            move |_, _| {
                parser_text_output.set("Arial".to_string());
            },
            (),
        )
    };

    // This is where we will have the user prompted to load in a file
    let upload_clicked_callback = use_callback(
        move |e: MouseEvent, _| {
            e.stop_propagation();
            on_upload_file_clicked();
        },
        (),
    );

    // This is the callback to get the file's contents and load it onto the Editor
    let file_picked_callback = {
        let text_model = Rc::clone(&text_model);
        use_callback(
            move |e: Event, _| {
                let text_model = (*text_model).borrow_mut().clone();
                let input: HtmlInputElement = e.target_unchecked_into();
                // gloo making the code readable and easy to implement
                let filelist = FileList::from(input.files().unwrap());
                let file = filelist.first().unwrap();
                let contents = gloo::file::futures::read_as_text(file);
                spawn_local(async move {
                    let contents = contents.await;

                    let contents = contents.expect("File contains invalid utf8"); // TODO: implement a file checker, will load in anything

                    text_model.set_value(&contents);
                })
            },
            (),
        )
    };

    html! {
        <div>
            <button onclick={on_assemble_clicked}>{ "Assemble" }</button>
            <button onclick={on_execute_clicked}> { "Execute" }</button>
            <button onclick={on_reset_clicked}>{ "Reset" }</button>
            // button tied to the input file element, which is hidden to be more clean
            <input type="button" value="Load File" onclick={upload_clicked_callback} />
            <input type="file" id="file_input" style="display: none;" accept=".txt,.asm,.mips" onchange={file_picked_callback} />
            // Pass in register data from emu core
            <Regview gp={(*datapath).borrow().registers}/>
            <SwimEditor link={codelink.clone()} text_model={(*text_model).borrow().clone()} />
            <button onclick={on_error_clicked}>{ "Click" }</button>
            <Console parsermsg={(*parser_text_output).clone()}/>
        </div>
    }
}

/**********************  Editor Component **********************/

#[derive(PartialEq, Properties)]
pub struct SwimEditorProps {
    pub text_model: TextModel,
    pub link: CodeEditorLink,
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

    let scrollbar = IEditorScrollbarOptions::default();
    scrollbar.set_always_consume_mouse_wheel(false.into());
    options.set_scrollbar(Some(&scrollbar));

    options
}

#[function_component]
pub fn SwimEditor(props: &SwimEditorProps) -> Html {
    html! {
        <CodeEditor classes={css!(r#"height: 70vh; width: 79vw;"#)} link={props.link.clone()} options={get_options()} model={props.text_model.clone()} />
    }
}

/**********************  "Console" Component **********************/
#[derive(Properties, PartialEq)]
pub struct Consoleprops {
    pub parsermsg: String,
}

/**********************  File I/O Function ***********************/
pub fn on_upload_file_clicked() {
    // log!(JsValue::from("Upload clicked!"));

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let file_input_elem = document
        .get_element_by_id("file_input")
        .expect("File input element with id \"file_input\" should exist.");

    let file_input_elem = file_input_elem
        .dyn_into::<HtmlInputElement>()
        .expect("Element should be an HtmlInputElement");

    // log!(JsValue::from("Before click"));
    // workaround for https://github.com/yewstack/yew/pull/3037 since it's not in 0.20
    spawn_local(async move {
        file_input_elem.click();
    });
    // log!(JsValue::from("After click"));
}

// /**********************  Objects for SWIM ***********************/
// // IMPORTANT!!! Since we're not using any external JS for the project,
// // wasm-bindgen is not needed for this. Otherwise, read
// // https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/getter-and-setter.html

// // Specifically for CurrExecHighlight Decoration
// pub struct CurrExecHighlight {
//     decoration: Array
// }

// impl CurrExecHighlight {
//     pub fn new(decoration: Array) -> CurrExecHighlight {
//         CurrExecHighlight { decoration: Array::new() }
//     }

//     // setter
//     pub fn set_decoration(&self) -> () {
//         self.decoration.set(0, JsValue {idx: val, _marker: val})
//     }

//     // getter
//     pub fn get_decoration(&self) -> () {
//         self.decoration.get(0);
//     }
// }

fn main() {
    yew::Renderer::<App>::new().render();
}
