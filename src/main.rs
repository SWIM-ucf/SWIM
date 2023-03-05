pub mod emulation_core;
pub mod parser;
#[cfg(test)]
pub mod tests;
pub mod ui;

use emulation_core::datapath::Datapath;
use emulation_core::mips::datapath::MipsDatapath;
use gloo::{console::log, dialogs::alert, file::FileList};
use js_sys::Object;
use monaco::{
    api::TextModel,
    sys::editor::{
        IEditorMinimapOptions, IEditorScrollbarOptions, IStandaloneEditorConstructionOptions,
    },
    yew::{CodeEditor, CodeEditorLink},
};
use parser::parser_assembler_main::parser;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
//use stylist::yew::*;
use ui::console::component::Console;
use ui::regview::component::Regview;
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew::{html, Html, Properties};
use yew_hooks::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // This contains the binary representation of "ori $s0, $zero, 12345", which
    // stores 12345 in register $s0.
    let code = String::from("ori $s0, $zero, 12345\n");
    let language = String::from("mips");

    let mut switch_view = 0;
    true.then(|| {
        switch_view += 1;
    });
    false.then(|| {
        switch_view += 1;
    });

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

    let clipboard = use_clipboard();

    // Setup the array that would store decorations applied to the
    // text model and initialize the options for it.
    let new_decor_array = js_sys::Array::new();
    let old_decor_array = js_sys::Array::new();

    // Setting up the options/parameters which
    // will highlight the executed line.
    // The delta decor does not need to be change,
    // the only parameter that will need to be changed is
    // the range. Note: This would be the case, but since
    // delta_decor has to be two different functions and
    // the Copy trait is not applied there they need to
    // be replicated in both.

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

    // This is where code is assembled and loaded into the emulation core's memory.
    let on_assemble_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();

        let new_decor_array = new_decor_array.clone();
        let old_decor_array = old_decor_array.clone();
        let delta_decor = monaco::sys::editor::IModelDecorationOptions::default();
        delta_decor.set_is_whole_line(true.into());
        delta_decor.set_inline_class_name("myInlineDecoration".into());
        use_callback(
            move |_, text_model| {
                let mut datapath = (*datapath).borrow_mut();
                let text_model = (*text_model).borrow_mut();

                // Pull ProgramInfo from the parser and parses through the code to assemble the binary
                let (programinfo, assembled) = parser(text_model.get_value());
                // Highlight the first line since the PC initializes on it.
                let list_of_line_numbers = programinfo.address_to_line_number;
                let index = datapath.registers.pc as usize / 4;
                let curr_line = *list_of_line_numbers.get(index).unwrap() as f64 + 1.0; // add one to account for the editor's line numbers
                let curr_model = text_model.as_ref();
                let curr_range = monaco::sys::Range::new(curr_line, 0.0, curr_line, 0.0);
                let highlight_line: monaco::sys::editor::IModelDeltaDecoration =
                    Object::new().unchecked_into();
                highlight_line.set_options(&delta_decor);
                let range_js = curr_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                highlight_line.set_range(&monaco::sys::IRange::from(range_js));
                let highlight_js = highlight_line
                    .dyn_into::<JsValue>()
                    .expect("Highlight is not found.");
                new_decor_array.push(&highlight_js);
                old_decor_array.set(
                    0,
                    (*curr_model)
                        .delta_decorations(&old_decor_array, &new_decor_array, None)
                        .into(),
                );
                // Load the binary into the datapath's memory
                (*datapath)
                    .initialize(assembled)
                    .expect("Memory could not be loaded");
                log!(datapath.memory.to_string());
                trigger.force_update();
                new_decor_array.pop();
            },
            text_model,
        )
    };

    // This is where the code will get executed. If you execute further
    // than when the code ends, the program crashes. As you execute the
    // code, the currently executed line is highlighted.
    let on_execute_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();

        let new_decor_array = new_decor_array.clone();
        let old_decor_array = old_decor_array.clone();
        let delta_decor = monaco::sys::editor::IModelDecorationOptions::default();
        delta_decor.set_is_whole_line(true.into());
        delta_decor.set_inline_class_name("myInlineDecoration".into());

        use_callback(
            move |_, _| {
                let mut datapath = (*datapath).borrow_mut();
                let text_model = (*text_model).borrow_mut();

                // Pull ProgramInfo from the parser
                let (programinfo, _) = parser(text_model.get_value());
                // Get the current line and convert it to f64
                let list_of_line_numbers = programinfo.address_to_line_number;
                let index = datapath.registers.pc as usize / 4;
                let curr_line = *list_of_line_numbers.get(index).unwrap() as f64 + 1.0; // add one to account for the editor's line numbers

                // Setup the range
                let curr_model = text_model.as_ref();
                let curr_range = monaco::sys::Range::new(curr_line, 0.0, curr_line, 0.0);

                // element to be stored in the Decoration array to highlight the line
                let highlight_line: monaco::sys::editor::IModelDeltaDecoration =
                    Object::new().unchecked_into();
                highlight_line.set_options(&delta_decor);
                let range_js = curr_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                highlight_line.set_range(&monaco::sys::IRange::from(range_js));
                let highlight_js = highlight_line
                    .dyn_into::<JsValue>()
                    .expect("Highlight is not found.");

                // log!("These are the arrays before the push");
                // log!(new_decor_array.at(0));
                // log!(old_decor_array.at(0));
                new_decor_array.push(&highlight_js);
                // it may look ugly, but it makes sense. Uncomment debug statements to see why.
                old_decor_array.set(
                    0,
                    (*curr_model)
                        .delta_decorations(&old_decor_array, &new_decor_array, None)
                        .into(),
                );
                (*datapath).execute_instruction();
                trigger.force_update();
                new_decor_array.pop(); // done with the highlight, prepare for the next one.
            },
            (),
        )
    };

    let on_execute_stage_clicked = {
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();
        use_callback(
            move |_, _| {
                let mut datapath = (*datapath).borrow_mut();
                (*datapath).execute_stage();
                trigger.force_update();
            },
            (),
        )
    };

    // This is how we will reset the datapath. This is the only method to "halt"
    // programs since if the user continues to execute, the whole application will
    // crash. This will also clear any highlight on the editor.
    let on_reset_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();

        let new_decor_array = new_decor_array;
        let old_decor_array = old_decor_array;

        use_callback(
            move |_, _| {
                let mut datapath = (*datapath).borrow_mut();
                let text_model = (*text_model).borrow_mut();
                let curr_model = text_model.as_ref();
                new_decor_array.pop();
                old_decor_array.set(
                    0,
                    (*curr_model)
                        .delta_decorations(&old_decor_array, &new_decor_array, None)
                        .into(),
                );
                (*datapath).reset();
                trigger.force_update();
            },
            (),
        )
    };

    let on_clipboard_clicked = {
        let text_model = Rc::clone(&text_model);
        let clipboard = clipboard;
        Callback::from(move |_: _| {
            let text_model = (*text_model).borrow_mut();
            clipboard.write_text(text_model.get_value());
            alert("Your code is saved to the clipboard.\nPaste it onto a text file to save it.\n(Ctrl/Cmd + V)");
        })
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
        <>
            // button tied to the input file element, which is hidden to be more clean
            <input type="file" id="file_input" style="display: none;" accept=".txt,.asm,.mips" onchange={file_picked_callback} />
            <div style="display: flex; flex-direction: row; flex-wrap: nowrap; height: 100vh; padding: 8px; gap: 8px;">
                // Left column
                <div style="flex-basis: 70%; display: flex; flex-direction: column; align-items: stretch;">
                    // Top buttons
                    <div>
                        <button class="button" onclick={on_assemble_clicked}>{ "Assemble" }</button>
                        <button class="button" onclick={on_execute_clicked} disabled={(*datapath).borrow().is_halted()}> { "Execute" }</button>
                        <button class="button" onclick={on_execute_stage_clicked} disabled={(*datapath).borrow().is_halted()}> { "Execute Stage" }</button>
                        <button class="button" onclick={on_reset_clicked}>{ "Reset" }</button>
                        <input type="button" value="Load File" onclick={upload_clicked_callback} />
                        <input type="button" value="Save to Clipboard" onclick={on_clipboard_clicked} />
                    </div>

                    // Editor
                    <div style="flex-grow: 1; min-height: 4em;">
                        <SwimEditor text_model={(*text_model).borrow().clone()} link={codelink.clone()} />
                    </div>

                    <div>
                        <button onclick={on_error_clicked}>{ "Click" }</button>
                    </div>

                    // Console
                    <Console parsermsg={(*parser_text_output).clone()} datapath={(*datapath.borrow()).clone()}/>
                </div>

                // Right column
                <Regview gp={(*datapath).borrow().registers} fp={(*datapath).borrow().coprocessor.fpr}/>
            </div>
        </>
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
        <CodeEditor classes={"editor"} options={get_options()} model={props.text_model.clone()} link={props.link.clone()} />
    }
}

/**********************  "Console" Component **********************/
#[derive(PartialEq, Properties)]
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

fn main() {
    yew::Renderer::<App>::new().render();
}
