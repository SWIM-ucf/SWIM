use gloo::{dialogs::alert, file::FileList};
use gloo_console::log;
use js_sys::Object;
use monaco::{
    api::TextModel,
    sys::{
        editor::{
            IEditorMinimapOptions, IEditorScrollbarOptions, IMarkerData, IModelDecorationOptions,
            IModelDeltaDecoration, IStandaloneEditorConstructionOptions, ISuggestOptions,
        },
        IMarkdownString, MarkerSeverity,
    },
    yew::CodeEditor,
};
use std::rc::Rc;
use swim::agent::datapath_communicator::DatapathCommunicator;
use swim::agent::datapath_reducer::DatapathReducer;
use swim::agent::EmulationCoreAgent;
use swim::emulation_core::datapath::Datapath;
use swim::emulation_core::mips::datapath::MipsDatapath;
use swim::emulation_core::mips::datapath::Stage;
use swim::parser::parser_assembler_main::parser;
use swim::ui::console::component::Console;
use swim::ui::regview::component::Regview;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Html, Properties};
use yew_agent::Spawnable;
use yew_hooks::prelude::*;

// To load in the Fibonacci example, uncomment the CONTENT and fib_model lines
// and comment the code, language, and text_model lines. IMPORTANT:
// rename fib_model to text_model to have it work.
const CONTENT: &str = include_str!("../../static/assembly_examples/fibonacci.asm");

#[derive(Properties, Clone, PartialEq)]
struct AppProps {
    communicator: &'static DatapathCommunicator,
}

#[function_component(App)]
fn app(props: &AppProps) -> Html {
    // This contains the binary representation of "ori $s0, $zero, 12345", which
    // stores 12345 in register $s0.
    // let code = String::from("ori $s0, $zero, 12345\n");
    // let language = String::from("mips");

    // This is the initial text model with default text contents. The
    // use_state_eq hook is created so that the component can be updated
    // when the text model changes.
    //let text_model = use_mut_ref(|| TextModel::create(&code, Some(&language), None).unwrap());
    let text_model = use_mut_ref(|| TextModel::create(CONTENT, Some("mips"), None).unwrap());

    // Setup the array that would store decorations applied to the
    // text model and initialize the options for it.
    let hover_jsarray = js_sys::Array::new();
    let hover_decor_array = use_mut_ref(js_sys::Array::new);

    // Setup the highlight stacks that would store which line
    // was executed after the execute button is pressed.
    let executed_line = js_sys::Array::new();
    let not_highlighted = js_sys::Array::new();

    // Setting up the options/parameters which
    // will highlight the previously executed line.
    // The highlight decor does not need to be changed,
    // the only parameter that will change is the range.
    let highlight_decor = use_mut_ref(monaco::sys::editor::IModelDecorationOptions::default);
    (*highlight_decor)
        .borrow_mut()
        .set_is_whole_line(true.into());
    (*highlight_decor)
        .borrow_mut()
        .set_inline_class_name("myInlineDecoration".into());

    // Output strings for the console and memory viewers.
    let parser_text_output = use_state_eq(String::new);
    let memory_text_output = use_state_eq(String::new);

    // Since we want the Datapath to be independent from all the
    // events within the app, we will create it when the app loads. This is also done
    // since the scope will be open across all events involved with it. To achieve this,
    // we use interior mutability to have the reference to the Datapath immutable, but
    // the ability to access and change its contents be mutable.
    let datapath = use_mut_ref(MipsDatapath::default);

    let datapath_state = use_reducer(DatapathReducer::default);

    // Start listening for messages from the communicator. This effectively links the worker thread to the main thread
    // and will force updates whenever its internal state changes.
    {
        let dispatcher = datapath_state.dispatcher();
        use_effect_with_deps(
            move |communicator| {
                spawn_local(communicator.listen_for_updates(dispatcher));
            },
            props.communicator,
        );
    }

    // This is where code is assembled and loaded into the emulation core's memory.
    let on_assemble_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let parser_text_output = parser_text_output.clone();
        let trigger = use_force_update();
        let communicator = props.communicator;

        let executed_line = executed_line.clone();
        let not_highlighted = not_highlighted.clone();

        use_callback(
            move |_, text_model| {
                let mut datapath = datapath.borrow_mut();
                let text_model = text_model.borrow_mut();

                // parses through the code to assemble the binary and retrieves programinfo for error marking and mouse hover
                let (program_info, assembled) = parser(text_model.get_value());
                parser_text_output.set(program_info.console_out_post_assembly);

                let mut markers: Vec<IMarkerData> = vec![];

                // Parse output from parser and create an instance of IMarkerData for each error.
                for (line_number, line_information) in
                    program_info.monaco_line_info.iter().enumerate()
                {
                    for error in &line_information.errors {
                        let new_marker: IMarkerData = new_object().into();
                        new_marker.set_message(&error.message);
                        new_marker.set_severity(MarkerSeverity::Error);
                        new_marker.set_start_line_number((line_number + 1) as f64);
                        new_marker.set_start_column((error.start_end_columns.0 + 1) as f64);
                        new_marker.set_end_line_number((line_number + 1) as f64);
                        new_marker.set_end_column((error.start_end_columns.1 + 1) as f64);
                        markers.push(new_marker);
                    }
                }

                // Convert Vec<IMarkerData> to Javascript array
                let marker_jsarray = js_sys::Array::new();
                for marker in markers {
                    marker_jsarray.push(&marker);
                }

                monaco::sys::editor::set_model_markers(
                    text_model.as_ref(),
                    "owner",
                    &marker_jsarray,
                );
                // Acts like reset and clears the highlight
                let curr_model = text_model.as_ref();
                executed_line.pop();
                not_highlighted.set(
                    0,
                    curr_model
                        .delta_decorations(&not_highlighted, &executed_line, None)
                        .into(),
                );

                // Proceed with loading into memory and expand pseudo-instructions if there are no errors.
                if marker_jsarray.length() == 0 {
                    // Load the binary into the datapath's memory
                    match datapath.initialize_legacy(assembled.clone()) {
                        Ok(_) => (),
                        Err(msg) => {
                            // In the case of an error, note this and stop early.
                            parser_text_output.set(format!("This program failed to load into the datapath. Message returned by datapath: {msg}"));
                        }
                    }
                    // log!(datapath.memory.to_string());
                    text_model.set_value(&program_info.updated_monaco_string); // Expands pseudo-instructions to their hardware counterpart.
                    datapath.registers.pc = program_info.pc_starting_point as u64;
                    // Send the binary over to the emulation core thread
                    communicator.initialize(program_info.pc_starting_point, assembled)
                }

                trigger.force_update();
            },
            text_model,
        )
    };

    log!("Re-rendered!");

    // This is where the code will get executed. If you execute further
    // than when the code ends, the program crashes. This is remedied via the
    // syscall instruction, which will halt the datapath. As you execute the
    // code, the previously executed line is highlighted.
    let on_execute_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();
        let communicator = props.communicator;

        let executed_line = executed_line.clone();
        let not_highlighted = not_highlighted.clone();
        let highlight_decor = highlight_decor.clone();

        use_callback(
            move |_, _| {
                let mut datapath = datapath.borrow_mut();
                let text_model = text_model.borrow_mut();
                let highlight_decor = highlight_decor.borrow_mut();

                // Pull ProgramInfo from the parser
                let (programinfo, _) = parser(text_model.get_value());

                // Get the current line and convert it to f64
                let list_of_line_numbers = programinfo.address_to_line_number;
                let index = datapath.registers.pc as usize / 4;
                let curr_line = *list_of_line_numbers.get(index).unwrap_or(&0) as f64 + 1.0; // add one to account for the editor's line numbers

                // Setup the range
                let curr_model = text_model.as_ref();
                let curr_range = monaco::sys::Range::new(curr_line, 0.0, curr_line, 0.0);

                // element to be stored in the stack to highlight the line
                let highlight_line: monaco::sys::editor::IModelDeltaDecoration =
                    Object::new().unchecked_into();
                highlight_line.set_options(&highlight_decor);
                let range_js = curr_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                highlight_line.set_range(&monaco::sys::IRange::from(range_js));
                let highlight_js = highlight_line
                    .dyn_into::<JsValue>()
                    .expect("Highlight is not found.");

                // log!("These are the stacks before the push");
                // log!(executed_line.at(0));
                // log!(not_highlighted.at(0));

                // push the decoration onto the executed_line stack
                executed_line.push(&highlight_js);

                // it may look ugly, but it makes sense. Uncomment debug statements to see why.
                not_highlighted.set(
                    0,
                    curr_model
                        .delta_decorations(&not_highlighted, &executed_line, None)
                        .into(),
                );

                // log!("These are the stacks after the push");
                // log!(executed_line.at(0));
                // log!(not_highlighted.at(0));

                datapath.execute_instruction();
                communicator.execute_instruction();

                // done with the highlight, prepare for the next one.
                executed_line.pop();

                // log!("These are the stacks after the pop");
                // log!(executed_line.at(0));
                // log!(not_highlighted.at(0));

                trigger.force_update();
            },
            (),
        )
    };

    let on_execute_stage_clicked = {
        let datapath = Rc::clone(&datapath);
        let text_model = Rc::clone(&text_model);
        let executed_line = executed_line.clone();
        let not_highlighted = not_highlighted.clone();
        let highlight_decor = highlight_decor;
        let trigger = use_force_update();
        let communicator = props.communicator;

        use_callback(
            move |_, _| {
                let mut datapath = datapath.borrow_mut();
                let highlight_decor = highlight_decor.borrow_mut();
                if datapath.current_stage == Stage::InstructionDecode {
                    // highlight on InstructionDecode since syscall stops at that stage.
                    let text_model = text_model.borrow_mut();
                    let (programinfo, _) = parser(text_model.get_value());
                    let list_of_line_numbers = programinfo.address_to_line_number;
                    let index = datapath.registers.pc as usize / 4;
                    let curr_line = *list_of_line_numbers.get(index).unwrap_or(&0) as f64 + 1.0;
                    let curr_model = text_model.as_ref();
                    let curr_range = monaco::sys::Range::new(curr_line, 0.0, curr_line, 0.0);
                    let highlight_line: monaco::sys::editor::IModelDeltaDecoration =
                        Object::new().unchecked_into();
                    highlight_line.set_options(&highlight_decor);
                    let range_js = curr_range
                        .dyn_into::<JsValue>()
                        .expect("Range is not found.");
                    highlight_line.set_range(&monaco::sys::IRange::from(range_js));
                    let highlight_js = highlight_line
                        .dyn_into::<JsValue>()
                        .expect("Highlight is not found.");
                    executed_line.push(&highlight_js);
                    not_highlighted.set(
                        0,
                        curr_model
                            .delta_decorations(&not_highlighted, &executed_line, None)
                            .into(),
                    );
                    datapath.execute_stage();
                    executed_line.pop();
                } else {
                    datapath.execute_stage();
                }
                communicator.execute_stage();
                trigger.force_update();
            },
            (),
        )
    };

    // This is how we will reset the datapath.
    // This will also clear any highlight on the editor.
    let on_reset_clicked = {
        let text_model = Rc::clone(&text_model);
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();
        let parser_text_output = parser_text_output.clone();
        let communicator = props.communicator;

        let executed_line = executed_line;
        let not_highlighted = not_highlighted;

        use_callback(
            move |_, _| {
                let mut datapath = datapath.borrow_mut();
                let text_model = text_model.borrow_mut();
                let curr_model = text_model.as_ref();
                executed_line.pop();
                not_highlighted.set(
                    0,
                    curr_model
                        .delta_decorations(&not_highlighted, &executed_line, None)
                        .into(),
                );
                parser_text_output.set("".to_string());
                datapath.reset();
                communicator.reset();
                trigger.force_update();
            },
            (),
        )
    };

    // Copies text to the user's clipboard
    let on_clipboard_clicked = {
        let text_model = Rc::clone(&text_model);
        let clipboard = use_clipboard();
        Callback::from(move |_: _| {
            let text_model = text_model.borrow_mut();
            clipboard.write_text(text_model.get_value());
            alert("Your code is saved to the clipboard.\nPaste it onto a text file to save it.\n(Ctrl/Cmd + V)");
        })
    };

    // We'll have the Mouse Hover event running at all times.
    {
        let text_model = Rc::clone(&text_model);
        use_event_with_window("mouseover", move |_: MouseEvent| {
            let hover_jsarray = hover_jsarray.clone();
            let hover_decor_array = hover_decor_array.clone();
            let text_model = text_model.borrow_mut();
            let curr_model = text_model.as_ref();
            let (program_info, _) = parser(text_model.get_value());

            // Parse output from parser and create an instance of IModelDeltaDecoration for each line.
            for (line_number, line_information) in program_info.monaco_line_info.iter().enumerate()
            {
                let decoration: IModelDeltaDecoration = new_object().into();

                let hover_range = monaco::sys::Range::new(
                    (line_number + 1) as f64,
                    0.0,
                    (line_number + 1) as f64,
                    0.0,
                );
                let hover_range_js = hover_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                decoration.set_range(&monaco::sys::IRange::from(hover_range_js));

                let hover_opts: IModelDecorationOptions = new_object().into();
                hover_opts.set_is_whole_line(true.into());
                let hover_message: IMarkdownString = new_object().into();
                js_sys::Reflect::set(
                    &hover_message,
                    &JsValue::from_str("value"),
                    &JsValue::from_str(&line_information.mouse_hover_string),
                )
                .unwrap();
                hover_opts.set_hover_message(&hover_message);
                decoration.set_options(&hover_opts);
                let hover_js = decoration
                    .dyn_into::<JsValue>()
                    .expect("Hover is not found.");
                hover_jsarray.push(&hover_js);
            }

            // log!("This is the array after the push");
            // log!(hover_jsarray.clone());

            // properly pass the handlers onto the array
            let new_hover_decor_array =
                curr_model.delta_decorations(&hover_decor_array.borrow_mut(), &hover_jsarray, None);
            *hover_decor_array.borrow_mut() = new_hover_decor_array;

            // log!("These are the arrays after calling Delta Decorations");
            // log!(hover_jsarray.clone());
            // log!(hover_decor_array.borrow_mut().clone());

            // empty out the array that hold the decorations
            hover_jsarray.set_length(0);

            // log!("These are the arrays after calling popping the hover_jsarray");
            // log!(hover_jsarray.clone());
            // log!(hover_decor_array.borrow_mut().clone());
        });
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
                let text_model = text_model.borrow_mut().clone();
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
                <div style="flex-basis: 70%; display: flex; flex-direction: column; align-items: stretch; min-width: 0;">
                    // Top buttons
                    <div>
                        <div class="buttons">
                            <button class="button" onclick={on_assemble_clicked}>{ "Assemble " }<i class="fa-sharp fa-solid fa-hammer"></i></button>
                            <button class="button" onclick={on_execute_clicked} disabled={datapath.borrow().is_halted()}>{ "Execute " }<i class="fa-regular fa-circle-play"></i></button>
                            <button class="button" onclick={on_execute_stage_clicked} disabled={datapath.borrow().is_halted()}> { "Execute Stage " }<i class="fa-solid fa-play"></i></button>
                            <button class="button" onclick={on_reset_clicked}>{ "Reset " }<i class="fa-solid fa-arrow-rotate-left"></i></button>
                            //<input type="button" value="Load File" onclick={upload_clicked_callback} />
                            <button class="button" onclick={upload_clicked_callback}>{"Upload File "}<i class="fa-sharp fa-solid fa-upload"></i></button>
                            //<input type="button" value="Save to Clipboard" onclick={on_clipboard_clicked} />
                            <button class="button" onclick={on_clipboard_clicked}>{"Copy to Clipboard "}<i class="fa-regular fa-copy"></i></button>
                        </div>
                    </div>

                    // Editor
                    <div style="flex-grow: 1; min-height: 4em;">
                        <SwimEditor text_model={text_model.borrow().clone()} />
                    </div>

                    // Console
                    <Console parsermsg={(*parser_text_output).clone()} datapath={(*datapath.borrow()).clone()}
                    memorymsg={(*memory_text_output).clone()}/>
                </div>

                // Right column
                <Regview gp={datapath_state.mips.registers} fp={datapath.borrow().coprocessor.fpr}/>
            </div>
        </>
    }
}

/// Creates a new `JsValue`.
fn new_object() -> JsValue {
    js_sys::Object::new().into()
}

/**********************  Editor Component **********************/

#[derive(PartialEq, Properties)]
pub struct SwimEditorProps {
    pub text_model: TextModel,
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

    let suggest = ISuggestOptions::default();
    suggest.set_show_keywords(false.into());
    suggest.set_show_variables(false.into());
    suggest.set_show_icons(false.into());
    suggest.set_show_words(false.into());
    suggest.set_filter_graceful(false.into());
    options.set_suggest(Some(&suggest));

    options
}

#[function_component]
pub fn SwimEditor(props: &SwimEditorProps) -> Html {
    html! {
        <CodeEditor classes={"editor"} options={get_options()} model={props.text_model.clone()} />
    }
}

/**********************  "Console" Component **********************/
#[derive(PartialEq, Properties)]
pub struct Consoleprops {
    pub parsermsg: String,
}

/**********************  File I/O Function ***********************/
pub fn on_upload_file_clicked() {
    // log!("Upload clicked!");

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let file_input_elem = document
        .get_element_by_id("file_input")
        .expect("File input element with id \"file_input\" should exist.");

    let file_input_elem = file_input_elem
        .dyn_into::<HtmlInputElement>()
        .expect("Element should be an HtmlInputElement");

    // log!("Before click");
    // workaround for https://github.com/yewstack/yew/pull/3037 since it's not in 0.20
    spawn_local(async move {
        file_input_elem.click();
    });
    // log!("After click");
}

fn main() {
    // Initialize and leak the communicator to ensure that the thread spawns immediately and the bridge to it lives
    // for the remainder of the program. We can use the communicator exclusively through immutable references for the
    // rest of the program.
    let bridge = EmulationCoreAgent::spawner().spawn("./worker.js");
    let communicator = Box::new(DatapathCommunicator::new(bridge));
    yew::Renderer::<App>::with_props(AppProps {
        communicator: Box::leak(communicator),
    })
    .render();
}
