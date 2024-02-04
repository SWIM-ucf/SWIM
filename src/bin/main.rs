use gloo::{dialogs::alert, file::FileList};
use log::debug;
// use monaco::sys::editor::IModelContentChangedEvent;
use gloo_console::log;
use js_sys::Object;
use monaco::{
    api::TextModel,
    sys::{
        editor::{
            IMarkerData,
            IModelDecorationOptions,
            IModelDeltaDecoration,
        },
        IMarkdownString, MarkerSeverity,
    }
};
use swim::{parser::parser_assembler_main::parser, ui::{console::component::TabState, swim_editor::component::EditorTabState}};
use swim::parser::parser_structs_and_enums::ProgramInfo;
use std::rc::Rc;
use swim::agent::EmulationCoreAgent;
use swim::agent::datapath_communicator::DatapathCommunicator;
use swim::emulation_core::datapath::Datapath;
use swim::emulation_core::mips::datapath::MipsDatapath;
use swim::emulation_core::mips::datapath::Stage;
use swim::ui::console::component::Console;
use swim::ui::regview::component::Regview;
use swim::ui::swim_editor::component::SwimEditor;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Html, Properties};
use log::Level;

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
    let text_model = use_state_eq(|| TextModel::create(CONTENT, Some("mips"), None).unwrap());

    // Setup the array that would store decorations applied to the
    // text model and initialize the options for it.
    let hover_jsarray = js_sys::Array::new();
    let hover_decor_array = use_mut_ref(js_sys::Array::new);

    // Store the currently executed line in code editor and hex editor
    let editor_curr_line = use_state_eq(|| 0.0);
    let memory_curr_line = use_state_eq(|| 0.0);

    // Output strings for the console and memory viewers.
    let parser_text_output = use_state_eq(String::new);
    let memory_text_output = use_state_eq(String::new);
    let pc_limit = use_state(|| 0);

    // Input strings from the code editor
    let lines_content = use_mut_ref(Vec::<String>::new);

    let program_info_ref = use_mut_ref(ProgramInfo::default);
    let binary_ref = use_mut_ref(Vec::<u32>::new);

    // let last_memory_text_model = use_mut_ref(|| TextModel::create(&memory_text_output, Some("ini"), None).unwrap());
    // let memory_text_model = use_mut_ref(|| TextModel::create(&memory_text_output, Some("ini"), None).unwrap());
    let last_memory_text_model = use_state_eq(|| TextModel::create(&memory_text_output, Some("ini"), None).unwrap());
    let memory_text_model = use_state_eq(|| TextModel::create(&memory_text_output, Some("ini"), None).unwrap());

    // Store the currently selected tabs in windows
    let console_active_tab = use_state_eq(TabState::default);
    let editor_active_tab = use_state_eq(EditorTabState::default);

    // Since we want the Datapath to be independent from all the
    // events within the app, we will create it when the app loads. This is also done
    // since the scope will be open across all events involved with it. To achieve this,
    // we use interior mutability to have the reference to the Datapath immutable, but
    // the ability to access and change its contents be mutable.
    let datapath = use_mut_ref(MipsDatapath::default);

    // Start listening for messages from the communicator. This effectively links the worker thread to the main thread
    // and will force updates whenever its internal state changes.
    {
        let trigger = use_force_update();
        use_effect_with_deps(move |communicator| {
            spawn_local(communicator.listen_for_updates(trigger));
        }, props.communicator);
    }

    // This is where code is assembled and loaded into the emulation core's memory.
    let on_assemble_clicked = {
        // props.communicator.send_test_message(1); // Test message, remove later.
        let communicator = props.communicator;
        // let text_model = Rc::clone(&text_model);
        let text_model = text_model.clone();
        // let memory_text_model = Rc::clone(&memory_text_model);
        let memory_text_model = memory_text_model.clone();
        // let last_memory_text_model = Rc::clone(&last_memory_text_model);
        let last_memory_text_model = last_memory_text_model.clone();
        let datapath = Rc::clone(&datapath);
        let parser_text_output = parser_text_output.clone();
        let trigger = use_force_update();
        let editor_curr_line = editor_curr_line.clone();

        // Clone the value before moving it into the closure
        let pc_limit = pc_limit.clone();
        let program_info_ref = Rc::clone(&program_info_ref);
        let binary_ref = Rc::clone(&binary_ref);

        use_callback(
            move |_, (text_model, editor_curr_line)| {
                let mut datapath = datapath.borrow_mut();
                let text_model = text_model.clone();
                let memory_text_model = memory_text_model.clone();
                // parses through the code to assemble the binary and retrieves programinfo for error marking and mouse hover
                let (program_info, assembled) = parser(text_model.get_value());
                *program_info_ref.borrow_mut() = program_info.clone();
                *binary_ref.borrow_mut() = assembled.clone();
                pc_limit.set(assembled.len() * 4);
                parser_text_output.set(program_info.console_out_post_assembly);
                let last_memory_text_model = last_memory_text_model.clone();

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

                // Reset highlighted line to 0
                editor_curr_line.set(0.0);

                // Proceed with loading into memory and expand pseudo-instructions if there are no errors.
                if marker_jsarray.length() == 0 {
                    // Load the binary into the datapath's memory
                    match datapath.initialize(assembled) {
                        Ok(_) => (),
                        Err(msg) => {
                            // In the case of an error, note this and stop early.
                            parser_text_output.set(format!("This program failed to load into the datapath. Message returned by datapath: {msg}"));
                        }
                    }
                    // log!(datapath.memory.to_string());
                    text_model.set_value(&program_info.updated_monaco_string); // Expands pseudo-instructions to their hardware counterpart.
                    let hexdump = &datapath.memory.generate_formatted_hex();
                    memory_text_model.set_value(hexdump);
                    last_memory_text_model.set_value(hexdump);
                    datapath.registers.pc = program_info.pc_starting_point as u64;
                }

                trigger.force_update();
            },
            (text_model, editor_curr_line),
        )
    };

    log!("Re-rendered!");

    // This is where the code will get executed. If you execute further
    // than when the code ends, the program crashes. This is remedied via the
    // syscall instruction, which will halt the datapath. As you execute the
    // code, the previously executed line is highlighted.
    let on_execute_clicked = {
        let datapath = Rc::clone(&datapath);

        // Code editor
        // let text_model = Rc::clone(&text_model);
        let text_model = text_model.clone();
        let editor_curr_line = editor_curr_line.clone();

        // Hex editor
        // let memory_text_model = Rc::clone(&memory_text_model);
        let memory_text_model = memory_text_model.clone();
        // let last_memory_text_model = Rc::clone(&last_memory_text_model);
        let last_memory_text_model = last_memory_text_model.clone();
        let trigger = use_force_update();

        use_callback(
            move |_, editor_curr_line| {
                let mut datapath = datapath.borrow_mut();
                let text_model = text_model.clone();
                // let memory_text_model = Rc::clone(&memory_text_model);
                let memory_text_model = memory_text_model.clone();
                // let last_memory_text_model = Rc::clone(&last_memory_text_model);
                let last_memory_text_model = last_memory_text_model.clone();
                // let mut editor_curr_line = editor_curr_line.borrow_mut();

                // Pull ProgramInfo from the parser
                let (programinfo, _) = parser(text_model.get_value());

                // Get the current line and convert it to f64
                let list_of_line_numbers = programinfo.address_to_line_number;
                let index = datapath.registers.pc as usize / 4;
                editor_curr_line.set(*list_of_line_numbers.get(index).unwrap_or(&0) as f64 + 1.0); // add one to account for the editor's line numbers

                // Execute instruction
                datapath.execute_instruction();

                // Update memory
                let hexdump = &datapath.memory.generate_formatted_hex();

                memory_text_model.set_value(hexdump);
                last_memory_text_model.set_value(hexdump);

                trigger.force_update();
            },
            editor_curr_line,
        )
    };

    let on_execute_stage_clicked = {
        let datapath = Rc::clone(&datapath);

        // Code editor
        // let text_model = Rc::clone(&text_model);
        let text_model = text_model.clone();
        let editor_curr_line = editor_curr_line.clone();
        let trigger = use_force_update();

        // Hex editor
        // let memory_text_model = Rc::clone(&memory_text_model);
        let memory_text_model = memory_text_model.clone();
        // let last_memory_text_model = Rc::clone(&last_memory_text_model);
        let last_memory_text_model = last_memory_text_model.clone();

        use_callback(
            move |_, editor_curr_line| {
                let mut datapath = datapath.borrow_mut();

                // let memory_text_model = Rc::clone(&memory_text_model);
                let memory_text_model = memory_text_model.clone();
                // let last_memory_text_model = Rc::clone(&last_memory_text_model);
                let last_memory_text_model = last_memory_text_model.clone();

                if datapath.current_stage == Stage::InstructionDecode {
                    // highlight on InstructionDecode since syscall stops at that stage.
                    let text_model = text_model.clone();
                    let (programinfo, _) = parser(text_model.get_value());
                    let list_of_line_numbers = programinfo.address_to_line_number;
                    let index = datapath.registers.pc as usize / 4;
                    editor_curr_line.set(*list_of_line_numbers.get(index).unwrap_or(&0) as f64 + 1.0);
                    datapath.execute_stage();
                } else {
                    datapath.execute_stage();
                }

                // Update memory
                let hexdump = &datapath.memory.generate_formatted_hex();

                memory_text_model.set_value(hexdump);
                last_memory_text_model.set_value(hexdump);
                trigger.force_update();
            },
            editor_curr_line,
        )
    };

    let on_memory_clicked = {
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();

        // Code editor
        // let text_model = Rc::clone(&text_model);
        let text_model = text_model.clone();

        // Hex editor
        // let memory_text_model = Rc::clone(&memory_text_model);
        let memory_text_model = memory_text_model.clone();
        // let last_memory_text_model = Rc::clone(&last_memory_text_model);
        let last_memory_text_model = last_memory_text_model.clone();

        use_callback(
            move |_, _| {
                let mut datapath = datapath.borrow_mut();
                let text_model = text_model.clone();

                let (programinfo, binary) = parser(text_model.get_value());

                // Update memory
                // let memory_text_model = Rc::clone(&memory_text_model);
                let memory_text_model = memory_text_model.clone();
                // let last_memory_text_model = Rc::clone(&last_memory_text_model);
                let last_memory_text_model = last_memory_text_model.clone();

                let last_memory_text_model_value = last_memory_text_model.get_value();
                let current_memory_text_model_value = memory_text_model.get_value();

                if current_memory_text_model_value != last_memory_text_model_value {
                    match datapath.memory.parse_hexdump(&current_memory_text_model_value) {
                        Ok(instructions) => {
                            debug!("Memory parsed with no errors.");
                            match datapath.memory.store_hexdump(instructions) {
                                Ok(changed_lines) => {
                                    debug!("Memory updated successfully. Changed lines:");
                                    debug!("{:?}", changed_lines);
                                    let mut lines_beyond_counter = programinfo.address_to_line_number.len();
                                    let mut curr_value = text_model.get_value().to_owned();
                                    let mut add_new_lines = false;
                                    for line in changed_lines {
                                        if line.line_number < programinfo.address_to_line_number.len() {
                                            debug!("{}", binary[line.line_number]);
                                            debug!("{}", programinfo.address_to_line_number[line.line_number]);
                                            let updated_line = programinfo.address_to_line_number[line.line_number] as f64 + 1.0;
                                            let curr_model = text_model.as_ref();

                                            let line_to_replace = curr_model.get_line_content(updated_line);
                                            let mut start_line_column = 0.0;
                                            let end_line_column = line_to_replace.len() as f64 + 2.0;
                                            for (i, c) in line_to_replace.chars().enumerate() {
                                                if c.is_alphanumeric() {
                                                    start_line_column = i as f64 + 1.0;
                                                    break;
                                                }
                                            }
                                            debug!("Line to replace -> {:?}, {:?}: {:?}: {:?}: {:?}", line_to_replace, updated_line, start_line_column, updated_line, end_line_column);

                                            let edit_range = monaco::sys::Range::new(updated_line, start_line_column, updated_line, end_line_column);
                                            let before_cursor_state = monaco::sys::Selection::new(updated_line, start_line_column, updated_line,end_line_column);
                                            let edit_operations: monaco::sys::editor::IIdentifiedSingleEditOperation = Object::new().unchecked_into();
                                            edit_operations.set_range(&edit_range);
                                            edit_operations.set_text(Some(&line.text));
                                            let edit_operations_array = js_sys::Array::new();
                                            edit_operations_array.push(&edit_operations);
                                            let before_cursor_state_array = js_sys::Array::new();
                                            before_cursor_state_array.push(&before_cursor_state);
                                            curr_model.push_edit_operations(&before_cursor_state_array, &edit_operations_array, None);
                                        } else if line.line_number == lines_beyond_counter {
                                            debug!("Adding new line: {}", &line.text);
                                            // check if we've added new lines already
                                            if !add_new_lines {
                                                // start adding new lines by getting a copy of the current text model to append to
                                                add_new_lines = true;
                                                curr_value = text_model.get_value();
                                            }
                                            curr_value.push_str("\n");
                                            curr_value.push_str(&line.text);
                                            lines_beyond_counter += 1;
                                        }
                                    }
                                    if add_new_lines {
                                        text_model.set_value(&curr_value);
                                    }

                                },
                                Err(err) => {
                                    debug!("Error: {}", err)
                                }
                            };
                            ()
                        },
                        Err(err) => {
                            debug!("Error updating memory: {}", err)
                        }
                    }
                }

                let hexdump = &datapath.memory.generate_formatted_hex();

                memory_text_model.set_value(hexdump);
                last_memory_text_model.set_value(hexdump);

                trigger.force_update();

            },
            (),
        )
    };

    // This is how we will reset the datapath.
    // This will also clear any highlight on the editor.
    let on_reset_clicked = {
        let datapath = Rc::clone(&datapath);
        let trigger = use_force_update();

        // Code editor
        let parser_text_output = parser_text_output.clone();
        let editor_curr_line = editor_curr_line.clone();

        // Hex editor
        // let memory_text_model = Rc::clone(&memory_text_model);
        let memory_text_model = memory_text_model.clone();
        // let last_memory_text_model = Rc::clone(&last_memory_text_model);
        let last_memory_text_model = last_memory_text_model.clone();

        use_callback(
            move |_, editor_curr_line| {
                let mut datapath = datapath.borrow_mut();

                // Set highlighted line to 0
                editor_curr_line.set(0.0);

                parser_text_output.set("".to_string());
                datapath.reset();

                // Clear hex editor content
                // let memory_text_model = Rc::clone(&memory_text_model);
                let memory_text_model = memory_text_model.clone();
                // let last_memory_text_model = Rc::clone(&last_memory_text_model);
                let last_memory_text_model = last_memory_text_model.clone();

                memory_text_model.set_value("");
                last_memory_text_model.set_value("");

                trigger.force_update();
            },
            editor_curr_line,
        )
    };

    // Copies text to the user's clipboard
    let on_clipboard_clicked = {
        // let text_model = Rc::clone(&text_model);
        let text_model = text_model.clone();
        let clipboard = use_clipboard();
        Callback::from(move |_: _| {
            let text_model = text_model.clone();
            clipboard.write_text(text_model.get_value());
            alert("Your code is saved to the clipboard.\nPaste it onto a text file to save it.\n(Ctrl/Cmd + V)");
        })
    };

    // We'll have the Mouse Hover event running at all times.
    {
        // let text_model = Rc::clone(&text_model);
        let text_model = text_model.clone();
        use_event_with_window("mouseover", move |_: MouseEvent| {
            let hover_jsarray = hover_jsarray.clone();
            let hover_decor_array = hover_decor_array.clone();
            let text_model = text_model.clone();
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
        // let text_model = Rc::clone(&text_model);
        let text_model = text_model.clone();
        use_callback(
            move |e: Event, _| {
                let text_model = text_model.clone();
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
                            <button class="button" onclick={on_memory_clicked}>{"Update Memory"}</button>
                        </div>
                    </div>

                    // Editor
                    <div class="code">
                        <SwimEditor text_model={text_model} lines_content={lines_content} program_info={program_info_ref.borrow().clone()} binary={binary_ref.borrow().clone()} memory_curr_line={memory_curr_line.clone()} editor_curr_line={editor_curr_line.clone()} active_tab={editor_active_tab.clone()} pc={(*datapath.borrow()).clone().registers.pc}/>
                    </div>

                    // Console
                    <Console parsermsg={(*parser_text_output).clone()} datapath={(*datapath.borrow()).clone()} memory_text_model={memory_text_model} memory_curr_line={memory_curr_line.clone()} active_tab={console_active_tab.clone()}/>
                </div>

                // Right column
                <Regview gp={datapath.borrow_mut().registers} fp={datapath.borrow().coprocessor.fpr} datapath={datapath} pc_limit={*pc_limit} communicator={props.communicator}/>
            </div>
        </>
    }
}

/// Creates a new `JsValue`.
fn new_object() -> JsValue {
    js_sys::Object::new().into()
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
    console_log::init_with_level(Level::Debug).unwrap();
    // Initialize and leak the communicator to ensure that the thread spawns immediately and the bridge to it lives
    // for the remainder of the program.
    let bridge = EmulationCoreAgent::spawner().spawn("./worker.js");
    let communicator = Box::new(DatapathCommunicator::new(bridge));
    yew::Renderer::<App>::with_props(AppProps {
        communicator: Box::leak(communicator),
    })
    .render();
}
