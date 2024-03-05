use gloo::{dialogs::alert, file::FileList};
use log::debug;
// use monaco::sys::editor::IModelContentChangedEvent;
use gloo_console::log;
use js_sys::Object;
use log::Level;
use monaco::{
    api::TextModel,
    sys::{editor::IMarkerData, MarkerSeverity},
};
use std::rc::Rc;
use swim::agent::datapath_reducer::DatapathReducer;
use swim::agent::EmulationCoreAgent;
use swim::emulation_core::mips::datapath::MipsDatapath;
use swim::emulation_core::mips::datapath::Stage;
use swim::parser::parser_assembler_main::parser;
use swim::parser::parser_structs_and_enums::ProgramInfo;
use swim::ui::footer::component::Footer;
use swim::ui::regview::component::Regview;
use swim::ui::swim_editor::component::SwimEditor;
use swim::{
    agent::datapath_communicator::DatapathCommunicator,
    parser::parser_structs_and_enums::Architecture,
};
use swim::{
    emulation_core::{architectures::AvailableDatapaths, mips::instruction::get_string_version},
    ui::{
        footer::component::FooterTabState,
        hex_editor::component::{parse_hexdump, UpdatedLine},
        swim_editor::component::EditorTabState,
    },
};
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
const CONTENT: &str = include_str!("../../static/assembly_examples/riscv_test.asm");
const ARCH: Architecture = Architecture::RISCV;

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

    // Store the currently executed line in code editor and hex editor
    let editor_curr_line = use_state_eq(|| 0.0);
    let memory_curr_instr = use_state_eq(|| 0);

    // Output strings for the console and memory viewers.
    let parser_text_output = use_state_eq(String::new);
    let memory_text_output = use_state_eq(String::new);
    let pc_limit = use_state(|| 0);

    // Input strings from the code editor
    let lines_content = use_mut_ref(Vec::<String>::new);

    let program_info_ref = use_mut_ref(ProgramInfo::default);
    let binary_ref = use_mut_ref(Vec::<u32>::new);

    let memory_text_model =
        use_state_eq(|| TextModel::create(&memory_text_output, Some("ini"), None).unwrap());

    // Show input
    let show_input = use_state_eq(bool::default);
    show_input.set(true);
    let command = use_state_eq(|| String::from("(Test) Enter a string"));

    // Store the currently selected tabs in windows
    let console_active_tab = use_state_eq(FooterTabState::default);
    let editor_active_tab = use_state_eq(EditorTabState::default);

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
        // props.communicator.send_test_message(1); // Test message, remove later.
        // let communicator = props.communicator;
        let text_model = text_model.clone();
        let memory_text_model = memory_text_model.clone();
        let memory_curr_instr = memory_curr_instr.clone();
        let datapath_state = datapath_state.clone();
        let parser_text_output = parser_text_output.clone();
        let trigger = use_force_update();
        let editor_curr_line = editor_curr_line.clone();
        let communicator = props.communicator;

        // Clone the value before moving it into the closure
        let pc_limit = pc_limit.clone();
        let program_info_ref = Rc::clone(&program_info_ref);
        let binary_ref = Rc::clone(&binary_ref);

        use_callback(
            move |_, (text_model, editor_curr_line, memory_curr_instr, datapath_state)| {
                let text_model = text_model.clone();
                let memory_text_model = memory_text_model.clone();
                // parses through the code to assemble the binary and retrieves programinfo for error marking and mouse hover
                let (program_info, assembled) = parser(text_model.get_value(), ARCH);
                *program_info_ref.borrow_mut() = program_info.clone();
                *binary_ref.borrow_mut() = assembled.clone();
                pc_limit.set(assembled.len() * 4);
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

                // Reset highlighted line to 0
                editor_curr_line.set(0.0);

                // Proceed with loading into memory and expand pseudo-instructions if there are no errors.
                if marker_jsarray.length() == 0 {
                    // Send the binary over to the emulation core thread
                    communicator.initialize(program_info.pc_starting_point, assembled);
                    memory_curr_instr.set(datapath_state.mips.registers.pc);
                    text_model.set_value(&program_info.updated_monaco_string); // Expands pseudo-instructions to their hardware counterpart.
                    let hexdump = &datapath_state.mips.memory.generate_formatted_hex();
                    memory_text_model.set_value(hexdump);
                }

                trigger.force_update();
            },
            (
                text_model,
                editor_curr_line,
                memory_curr_instr,
                datapath_state,
            ),
        )
    };

    log!("Re-rendered!");

    // This is where the code will get executed. If you execute further
    // than when the code ends, the program crashes. This is remedied via the
    // syscall instruction, which will halt the datapath. As you execute the
    // code, the previously executed line is highlighted.
    let on_execute_clicked = {
        let datapath_state = datapath_state.clone();
        let program_info_ref = Rc::clone(&program_info_ref);

        // Code editor
        let editor_curr_line = editor_curr_line.clone();
        let memory_curr_instr = memory_curr_instr.clone();

        // Hex editor
        let memory_text_model = memory_text_model.clone();

        let trigger = use_force_update();
        let communicator = props.communicator;

        use_callback(
            move |_, (editor_curr_line, memory_curr_instr, datapath_state)| {
                let memory_text_model = memory_text_model.clone();

                // Get the current line and convert it to f64
                let programinfo = Rc::clone(&program_info_ref);
                let programinfo = programinfo.borrow().clone();
                let list_of_line_numbers = programinfo.address_to_line_number;
                let index = datapath_state.mips.registers.pc as usize / 4;
                editor_curr_line.set(*list_of_line_numbers.get(index).unwrap_or(&0) as f64 + 1.0); // add one to account for the editor's line numbers
                memory_curr_instr.set(datapath_state.mips.registers.pc);

                // Execute instruction
                communicator.execute_instruction();

                // Update memory
                let hexdump = &datapath_state.mips.memory.generate_formatted_hex();

                memory_text_model.set_value(hexdump);

                trigger.force_update();
            },
            (editor_curr_line, memory_curr_instr, datapath_state),
        )
    };

    let on_execute_stage_clicked = {
        let datapath_state = datapath_state.clone();
        let program_info_ref = Rc::clone(&program_info_ref);
        let communicator = props.communicator;

        // Code editor
        let editor_curr_line = editor_curr_line.clone();

        // Hex editor
        let memory_text_model = memory_text_model.clone();
        let memory_curr_instr = memory_curr_instr.clone();

        let trigger = use_force_update();

        use_callback(
            move |_, (editor_curr_line, memory_curr_instr, datapath_state)| {
                let memory_text_model = memory_text_model.clone();

                if datapath_state.mips.current_stage == Stage::InstructionDecode {
                    // highlight on InstructionDecode since syscall stops at that stage.
                    let programinfo = Rc::clone(&program_info_ref);
                    let programinfo = programinfo.borrow().clone();
                    let list_of_line_numbers = programinfo.address_to_line_number;
                    let index = datapath_state.mips.registers.pc as usize / 4;
                    editor_curr_line
                        .set(*list_of_line_numbers.get(index).unwrap_or(&0) as f64 + 1.0);
                    memory_curr_instr.set(datapath_state.mips.registers.pc);
                    communicator.execute_stage();
                } else {
                    communicator.execute_stage();
                }

                // Update memory
                let hexdump = &datapath_state.mips.memory.generate_formatted_hex();

                memory_text_model.set_value(hexdump);

                trigger.force_update();
            },
            (editor_curr_line, memory_curr_instr, datapath_state),
        )
    };

    let on_memory_clicked = {
        let program_info_ref = Rc::clone(&program_info_ref);

        // Code editor
        let text_model = text_model.clone();

        // Hex editor
        let memory_text_model = memory_text_model.clone();

        let trigger = use_force_update();
        let communicator = props.communicator;
        let datapath_state = datapath_state.clone();

        use_callback(
            move |_, datapath_state| {
                let text_model = text_model.clone();

                let program_info_ref = Rc::clone(&program_info_ref);

                // Update memory
                let memory_text_model = memory_text_model.clone();

                let current_memory_text_model_value = memory_text_model.get_value();

                match parse_hexdump(&current_memory_text_model_value) {
                    Ok(instructions) => {
                        let mut changed_lines: Vec<UpdatedLine> = vec![];
                        for (i, data) in instructions.iter().enumerate() {
                            let address = i as u64;
                            // change string version based on architecture
                            let string_version = match datapath_state.current_architecture {
                                AvailableDatapaths::MIPS => match get_string_version(*data) {
                                    Ok(string) => string,
                                    Err(string) => string,
                                },
                                AvailableDatapaths::RISCV => String::from(""),
                            };

                            let curr_word = match datapath_state.mips.memory.load_word(address * 4)
                            {
                                Ok(data) => data,
                                Err(e) => {
                                    debug!("{:?}", e);
                                    0
                                }
                            };
                            if curr_word != *data {
                                changed_lines.push(UpdatedLine::new(string_version, i));

                                communicator.set_memory(address * 4, *data);
                            }
                        }
                        // Memory updated successfully
                        let program_info = program_info_ref.borrow().clone();
                        let mut lines_beyond_counter = program_info.address_to_line_number.len();
                        let mut curr_value = text_model.get_value();
                        let mut add_new_lines = false;
                        for line in changed_lines {
                            // Check if we're updating or appending instruction
                            if line.line_number < program_info.address_to_line_number.len() {
                                let updated_line =
                                    program_info.address_to_line_number[line.line_number] as f64
                                        + 1.0;
                                let curr_model = text_model.as_ref();

                                // Get the current line's contents in the code editor
                                let line_to_replace = curr_model.get_line_content(updated_line);
                                // Create the range to replace
                                let mut start_line_column = 0.0;
                                let end_line_column = line_to_replace.len() as f64 + 2.0;
                                for (i, c) in line_to_replace.chars().enumerate() {
                                    if c.is_alphanumeric() {
                                        start_line_column = i as f64 + 1.0;
                                        break;
                                    }
                                }
                                let edit_range = monaco::sys::Range::new(
                                    updated_line,
                                    start_line_column,
                                    updated_line,
                                    end_line_column,
                                );
                                let before_cursor_state = monaco::sys::Selection::new(
                                    updated_line,
                                    start_line_column,
                                    updated_line,
                                    end_line_column,
                                );
                                // Create the edit operation using the range and new text
                                let edit_operations: monaco::sys::editor::IIdentifiedSingleEditOperation = Object::new().unchecked_into();
                                edit_operations.set_range(&edit_range);
                                edit_operations.set_text(Some(&line.text));
                                // Append it to JavaScript Array
                                let edit_operations_array = js_sys::Array::new();
                                edit_operations_array.push(&edit_operations);
                                let before_cursor_state_array = js_sys::Array::new();
                                before_cursor_state_array.push(&before_cursor_state);
                                // Do the edit!
                                curr_model.push_edit_operations(
                                    &before_cursor_state_array,
                                    &edit_operations_array,
                                    None,
                                );
                            } else if line.line_number == lines_beyond_counter {
                                // Append instruction
                                if !add_new_lines {
                                    // If we've added new lines already,
                                    // start adding new lines by getting a copy of the current text model to append to
                                    add_new_lines = true;
                                    curr_value = text_model.get_value();
                                }
                                curr_value.push('\n');
                                curr_value.push_str(&line.text);
                                lines_beyond_counter += 1;
                            }
                        }
                        if add_new_lines {
                            text_model.set_value(&curr_value);
                        }
                    }
                    Err(err) => {
                        debug!("Error updating memory: {}", err)
                    }
                }

                // Update the parsed info for text and data segment views
                let (program_info, _) = parser(text_model.get_value(), ARCH);
                *program_info_ref.borrow_mut() = program_info;

                trigger.force_update();
            },
            datapath_state,
        )
    };

    // This is how we will reset the datapath.
    // This will also clear any highlight on the editor.
    let on_reset_clicked = {
        let datapath_state = datapath_state.clone();
        let trigger = use_force_update();
        let parser_text_output = parser_text_output.clone();
        let communicator = props.communicator;

        // Code editor
        let parser_text_output = parser_text_output;
        let editor_curr_line = editor_curr_line.clone();

        // Hex editor
        let memory_text_model = memory_text_model.clone();
        let memory_curr_instr = memory_curr_instr.clone();

        use_callback(
            move |_, (editor_curr_line, datapath_state)| {
                // Set highlighted line to 0
                editor_curr_line.set(0.0);
                memory_curr_instr.set(datapath_state.mips.registers.pc);

                parser_text_output.set("".to_string());
                communicator.reset();

                // Clear hex editor content
                let memory_text_model = memory_text_model.clone();

                memory_text_model.set_value("");

                communicator.reset();
                trigger.force_update();
            },
            (editor_curr_line, datapath_state),
        )
    };

    // Copies text to the user's clipboard
    let on_clipboard_clicked = {
        let text_model = text_model.clone();
        let clipboard = use_clipboard();
        Callback::from(move |_: _| {
            let text_model = text_model.clone();
            clipboard.write_text(text_model.get_value());
            alert("Your code is saved to the clipboard.\nPaste it onto a text file to save it.\n(Ctrl/Cmd + V)");
        })
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
                            <button class="button" onclick={on_execute_clicked} disabled={false}>{ "Execute " }<i class="fa-regular fa-circle-play"></i></button>
                            <button class="button" onclick={on_execute_stage_clicked} disabled={false}> { "Execute Stage " }<i class="fa-solid fa-play"></i></button>
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
                        <SwimEditor text_model={text_model} lines_content={lines_content} program_info={program_info_ref.borrow().clone()} pc_limit={*pc_limit} binary={binary_ref.borrow().clone()} memory_curr_instr={memory_curr_instr.clone()} editor_curr_line={editor_curr_line.clone()} editor_active_tab={editor_active_tab.clone()} console_active_tab={console_active_tab.clone()} pc={datapath_state.mips.registers.pc}/>
                    </div>

                    // Console
                    <Footer parsermsg={(*parser_text_output).clone()} datapath={(*datapath.borrow()).clone()} memory_text_model={memory_text_model} memory_curr_instr={memory_curr_instr.clone()} active_tab={console_active_tab.clone()} communicator={props.communicator} show_input={show_input.clone()} command={command.clone()}/>
                </div>

                // Right column
                <Regview gp={datapath_state.mips.registers} fp={datapath_state.mips.coprocessor_registers} pc_limit={*pc_limit} communicator={props.communicator}/>
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
