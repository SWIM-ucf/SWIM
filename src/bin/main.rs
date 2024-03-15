use gloo::file::FileList;
use gloo_console::log;
use js_sys::Object;
use log::debug;
use log::Level;
use monaco::{
    api::TextModel,
    sys::{editor::IMarkerData, MarkerSeverity},
};
use std::rc::Rc;
use swim::agent::datapath_communicator::DatapathCommunicator;
use swim::agent::datapath_reducer::DatapathReducer;
use swim::agent::EmulationCoreAgent;
use swim::emulation_core::mips::datapath::Stage;
use swim::parser::parser_assembler_main::parser;
use swim::parser::parser_structs_and_enums::ProgramInfo;
use swim::ui::footer::component::Footer;
use swim::ui::regview::component::Regview;
use swim::ui::swim_editor::component::SwimEditor;
use swim::{
    emulation_core::{
        architectures::AvailableDatapaths, mips::gp_registers::GpRegisterType,
        mips::instruction::get_string_version,
    },
    ui::{
        hex_editor::component::{parse_hexdump, UpdatedLine},
        swim_editor::tab::TabState,
    },
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Html, Properties};

use yew_agent::Spawnable;

// To load in the Fibonacci example, uncomment the CONTENT and fib_model lines
// and comment the code, language, and text_model lines. IMPORTANT:
// rename fib_model to text_model to have it work.
const CONTENT: &str = include_str!("../../static/assembly_examples/fibonacci.asm");
const ARCH: AvailableDatapaths = AvailableDatapaths::MIPS;

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
    let console_active_tab = use_state_eq(|| TabState::Console);
    let editor_active_tab = use_state_eq(|| TabState::Editor);

    let datapath_state = use_reducer(DatapathReducer::default);

    // Start listening for messages from the communicator. This effectively links the worker thread to the main thread
    // and will force updates whenever its internal state changes.
    {
        let dispatcher = datapath_state.dispatcher();
        use_effect_with_deps(
            move |communicator: &&DatapathCommunicator| {
                spawn_local(communicator.listen_for_updates(dispatcher));
            },
            props.communicator,
        );
    }

    // This is where code is assembled and loaded into the emulation core's memory.
    let on_assemble_clicked = {
        let text_model = text_model.clone();
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

        let trigger = use_force_update();
        let communicator = props.communicator;

        use_callback(
            move |_, (editor_curr_line, memory_curr_instr, datapath_state)| {
                // Get the current line and convert it to f64
                let programinfo = Rc::clone(&program_info_ref);
                let programinfo = programinfo.borrow().clone();
                let list_of_line_numbers = programinfo.address_to_line_number;
                let index = datapath_state.mips.registers.pc as usize / 4;
                editor_curr_line.set(*list_of_line_numbers.get(index).unwrap_or(&0) as f64 + 1.0); // add one to account for the editor's line numbers
                memory_curr_instr.set(datapath_state.mips.registers.pc);

                // Execute instruction
                communicator.execute_instruction();

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
        let memory_curr_instr = memory_curr_instr.clone();

        let trigger = use_force_update();

        use_callback(
            move |_, (editor_curr_line, memory_curr_instr, datapath_state)| {
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

                trigger.force_update();
            },
            (editor_curr_line, memory_curr_instr, datapath_state),
        )
    };

    let on_continue_execution = {
        let communicator = props.communicator;
        use_callback(
            move |_, _| {
                communicator.execute();
            },
            (),
        )
    };

    let on_pause_execution = {
        let communicator = props.communicator;
        use_callback(
            move |_, _| {
                communicator.pause_core();
            },
            (),
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
        let trigger = use_force_update();
        let parser_text_output = parser_text_output.clone();
        let communicator = props.communicator;

        // Code editor
        let parser_text_output = parser_text_output;
        let editor_curr_line = editor_curr_line.clone();

        // Hex editor
        let memory_curr_instr = memory_curr_instr.clone();

        use_callback(
            move |_, editor_curr_line| {
                // Set highlighted line to 0
                editor_curr_line.set(0.0);
                memory_curr_instr.set(0);

                parser_text_output.set("".to_string());
                communicator.reset();

                communicator.reset();
                trigger.force_update();
            },
            editor_curr_line,
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
            <input type="file" class="hidden" id="file_input" accept=".txt,.asm,.mips" onchange={file_picked_callback} />
            <div class="flex flex-row flex-no-wrap h-screen p-2 gap-2">
                // Left column
                <div class="flex basis-3/4 flex-col items-stretch min-w-0">
                    // Top buttons
                    <div>
                        <div class="my-0 mx-auto flex flex-row items-center gap-2 overflow-visible">
                            <button class="opacity-90 duration-300 border-0 border-r-4 border solid border-primary-300 pr-2 " title="Upload file" onclick={upload_clicked_callback}>
                                <svg width="32" height="28" viewBox="0 0 32 28" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M18 4.30769L14 0H0V28H32V4.30769H18ZM16 11.8462L23 19.3846H18V28H14V19.3846H9L16 11.8462Z" fill="#BBBBBB"/>
                                </svg>
                            </button>
                            <button class="group disabled:opacity-30 duration-300 " title="Assemble" onclick={on_assemble_clicked}>
                                <svg width="38" height="38" viewBox="0 0 38 38" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path class="group-hover:stroke-primary-100 group-hover:fill-primary-100" fill-rule="evenodd" clip-rule="evenodd" d="M34.1794 19.1007C34.1794 23.0891 32.595 26.9142 29.7748 29.7345C26.9545 32.5547 23.1294 34.1392 19.141 34.1392C15.1525 34.1392 11.3274 32.5547 8.50714 29.7345C5.68688 26.9142 4.10247 23.0891 4.10247 19.1007C4.10247 15.1122 5.68688 11.2871 8.50714 8.46686C11.3274 5.6466 15.1525 4.06219 19.141 4.06219C23.1294 4.06219 26.9545 5.6466 29.7748 8.46686C32.595 11.2871 34.1794 15.1122 34.1794 19.1007ZM34.1183 17.5507L36.2416 19.1007L34.1183 20.4552L35.9114 22.4366L33.5523 23.3059L34.941 25.6444L32.4369 25.9915L33.3606 28.6029L30.6009 28.8385L31.2344 31.1941L28.5455 30.894L28.6432 33.3203L26.1295 32.508L25.6847 34.9007L23.4439 33.6234L22.4768 35.8711L20.5932 34.0981L19.141 36.2013L17.6887 34.0981L15.8051 35.8711L14.838 33.532L12.5972 34.9007L12.1524 32.4167L9.63871 33.3203L9.5403 30.8617L7.04749 31.1941L7.48485 28.8063L4.92128 28.6029L5.87082 26.3903L3.34095 25.6444L4.75548 23.7047L2.3705 22.4366L4.18939 20.4552L2.04028 19.1007L4.18939 17.5507L2.3705 15.7648L4.75548 14.7L3.34095 12.5569L5.87082 12.0144L4.92128 9.59843H7.48485L7.04749 7.00721L9.64964 7.44457L9.63871 4.881L12.0656 5.83053L12.5972 3.30066L14.7512 4.7152L15.8051 2.33022L17.6019 4.14911L19.141 2L20.4564 3.97315L22.4768 2.33022L23.4161 4.30201L25.6847 3.30066L26.2272 5.83053L28.6432 4.881V7.44457L31.2344 7.00721L30.6986 9.50002L33.3606 9.59843L32.3127 11.916L34.941 12.5569L33.5523 14.7L35.9114 15.7648L34.1183 17.5507Z" stroke="#BBBBBB" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path class="group-hover:stroke-primary-100 group-hover:fill-primary-100" d="M27.4664 24.3107L22.6243 19.1458C22.3246 18.8261 21.8341 18.8261 21.5343 19.1458L21.171 19.5333L18.2164 16.3818L23.0667 11.2081H17.9283L15.6471 13.6413L15.4214 13.4005H14.3314V14.5632L14.5571 14.804L11.2483 18.3334L13.8175 21.0739L17.1263 17.5445L20.0809 20.6961L19.7176 21.0836C19.4178 21.4034 19.4178 21.9266 19.7176 22.2463L24.5597 27.4112C24.8594 27.731 25.3499 27.731 25.6497 27.4112L27.4664 25.4734C27.7662 25.1537 27.7662 24.6305 27.4664 24.3107Z" fill="#BBBBBB"/>
                                </svg>
                            </button>
                            <button class="group hover:stroke-primary-100 disabled:opacity-30 duration-300 " title="Execute" onclick={on_continue_execution} disabled={datapath_state.executing || !datapath_state.initialized}>
                                <svg width="38" height="38" viewBox="0 0 38 38" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path class="fill-accent-red-200 stroke-accent-red-200 group-hover:group-enabled:stroke-accent-red-100 group-hover:group-enabled:fill-accent-red-100" fill-rule="evenodd" clip-rule="evenodd" d="M33.9311 19.1007C33.9311 23.0891 32.3467 26.9142 29.5265 29.7345C26.7062 32.5547 22.8811 34.1392 18.8927 34.1392C14.9042 34.1392 11.0791 32.5547 8.25885 29.7345C5.43859 26.9142 3.85418 23.0891 3.85418 19.1007C3.85418 15.1122 5.43859 11.2871 8.25885 8.46686C11.0791 5.6466 14.9042 4.06219 18.8927 4.06219C22.8811 4.06219 26.7062 5.6466 29.5265 8.46686C32.3467 11.2871 33.9311 15.1122 33.9311 19.1007ZM33.8701 17.5507L35.9933 19.1007L33.8701 20.4552L35.6631 22.4366L33.304 23.3059L34.6927 25.6444L32.1886 25.9915L33.1123 28.6029L30.3526 28.8385L30.9861 31.1941L28.2972 30.894L28.3949 33.3203L25.8812 32.508L25.4364 34.9007L23.1956 33.6234L22.2286 35.8711L20.345 34.0981L18.8927 36.2013L17.4404 34.0981L15.5568 35.8711L14.5897 33.532L12.3489 34.9007L11.9041 32.4167L9.39042 33.3203L9.29201 30.8617L6.7992 31.1941L7.23656 28.8063L4.67299 28.6029L5.62253 26.3903L3.09265 25.6444L4.50719 23.7047L2.12221 22.4366L3.9411 20.4552L1.79199 19.1007L3.9411 17.5507L2.12221 15.7648L4.50719 14.7L3.09265 12.5569L5.62253 12.0144L4.67299 9.59843H7.23656L6.7992 7.00721L9.40135 7.44457L9.39042 4.881L11.8173 5.83053L12.3489 3.30066L14.5029 4.7152L15.5568 2.33022L17.3536 4.14911L18.8927 2L20.2081 3.97315L22.2286 2.33022L23.1678 4.30201L25.4364 3.30066L25.9789 5.83053L28.3949 4.881V7.44457L30.9861 7.00721L30.4504 9.50002L33.1123 9.59843L32.0644 11.916L34.6927 12.5569L33.304 14.7L35.6631 15.7648L33.8701 17.5507Z" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path class="fill-accent-red-200 stroke-accent-red-200 group-hover:group-enabled:stroke-accent-red-100 group-hover:group-enabled:fill-accent-red-100" d="M15.1309 13.536C15.1309 13.1414 15.5664 12.9023 15.8993 13.1142L24.6438 18.6789C24.9526 18.8753 24.9526 19.326 24.6438 19.5225L15.8993 25.0872C15.5664 25.2991 15.1309 25.0599 15.1309 24.6654V13.536Z" stroke-width="3"/>
                                </svg>
                            </button>
                            <button class="disabled:opacity-30 group duration-300 " title="Pause Execution" disabled={! datapath_state.executing} onclick={on_pause_execution}>
                                <svg width="38" height="38" viewBox="0 0 38 38" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path class="fill-accent-green-200 stroke-accent-green-200 group-hover:group-enabled:stroke-accent-green-100 group-hover:group-enabled:fill-accent-green-100" fill-rule="evenodd" clip-rule="evenodd" d="M34.186 19.1007C34.186 23.0891 32.6016 26.9142 29.7814 29.7345C26.9611 32.5547 23.136 34.1392 19.1475 34.1392C15.1591 34.1392 11.334 32.5547 8.51373 29.7345C5.69347 26.9142 4.10906 23.0891 4.10906 19.1007C4.10906 15.1122 5.69347 11.2871 8.51373 8.46686C11.334 5.6466 15.1591 4.06219 19.1475 4.06219C23.136 4.06219 26.9611 5.6466 29.7814 8.46686C32.6016 11.2871 34.186 15.1122 34.186 19.1007ZM34.1249 17.5507L36.2482 19.1007L34.1249 20.4552L35.918 22.4366L33.5589 23.3059L34.9476 25.6444L32.4435 25.9915L33.3672 28.6029L30.6075 28.8385L31.241 31.1941L28.5521 30.894L28.6498 33.3203L26.1361 32.508L25.6913 34.9007L23.4505 33.6234L22.4834 35.8711L20.5998 34.0981L19.1475 36.2013L17.6953 34.0981L15.8117 35.8711L14.8446 33.532L12.6038 34.9007L12.159 32.4167L9.6453 33.3203L9.54689 30.8617L7.05409 31.1941L7.49145 28.8063L4.92788 28.6029L5.87741 26.3903L3.34754 25.6444L4.76208 23.7047L2.3771 22.4366L4.19599 20.4552L2.04688 19.1007L4.19599 17.5507L2.3771 15.7648L4.76208 14.7L3.34754 12.5569L5.87741 12.0144L4.92788 9.59843H7.49144L7.05409 7.00721L9.65624 7.44457L9.6453 4.881L12.0722 5.83053L12.6038 3.30066L14.7578 4.7152L15.8117 2.33022L17.6085 4.14911L19.1475 2L20.463 3.97315L22.4834 2.33022L23.4227 4.30201L25.6913 3.30066L26.2338 5.83053L28.6498 4.881V7.44457L31.241 7.00721L30.7052 9.50002L33.3672 9.59843L32.3193 11.916L34.9476 12.5569L33.5589 14.7L35.918 15.7648L34.1249 17.5507Z" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path class="fill-accent-green-200 stroke-accent-green-200 group-hover:group-enabled:stroke-accent-green-100 group-hover:group-enabled:fill-accent-green-100" d="M13.228 13.1812H18.435V25.6779H13.228V13.1812ZM20.5177 13.1812H25.7247V25.6779H20.5177V13.1812Z"/>
                                </svg>
                            </button>
                            <button class="disabled:opacity-30 group duration-300 " title="Execute Next Stage" onclick={on_execute_stage_clicked} disabled={!datapath_state.initialized}>
                                <svg width="38" height="38" viewBox="0 0 38 38" xmlns="http://www.w3.org/2000/svg">
                                    <path class="stroke-primary-200 fill-transparent group-enabled:group-hover:stroke-primary-100"  fill-rule="evenodd" clip-rule="evenodd" d="M33.6829 19.1007C33.6829 23.0891 32.0984 26.9142 29.2782 29.7345C26.4579 32.5547 22.6328 34.1392 18.6444 34.1392C14.6559 34.1392 10.8308 32.5547 8.01056 29.7345C5.1903 26.9142 3.60589 23.0891 3.60589 19.1007C3.60589 15.1122 5.1903 11.2871 8.01056 8.46686C10.8308 5.6466 14.6559 4.06219 18.6444 4.06219C22.6328 4.06219 26.4579 5.6466 29.2782 8.46686C32.0984 11.2871 33.6829 15.1122 33.6829 19.1007ZM33.6218 17.5507L35.745 19.1007L33.6218 20.4552L35.4148 22.4366L33.0557 23.3059L34.4444 25.6444L31.9403 25.9915L32.864 28.6029L30.1043 28.8385L30.7378 31.1941L28.0489 30.894L28.1466 33.3203L25.6329 32.508L25.1881 34.9007L22.9473 33.6234L21.9803 35.8711L20.0967 34.0981L18.6444 36.2013L17.1921 34.0981L15.3085 35.8711L14.3414 33.532L12.1006 34.9007L11.6558 32.4167L9.14213 33.3203L9.04372 30.8617L6.55091 31.1941L6.98827 28.8063L4.4247 28.6029L5.37424 26.3903L2.84436 25.6444L4.2589 23.7047L1.87392 22.4366L3.69281 20.4552L1.5437 19.1007L3.69281 17.5507L1.87392 15.7648L4.2589 14.7L2.84436 12.5569L5.37424 12.0144L4.4247 9.59843H6.98827L6.55091 7.00721L9.15306 7.44457L9.14213 4.881L11.5691 5.83053L12.1006 3.30066L14.2546 4.7152L15.3085 2.33022L17.1053 4.14911L18.6444 2L19.9598 3.97315L21.9803 2.33022L22.9195 4.30201L25.1881 3.30066L25.7306 5.83053L28.1466 4.881V7.44457L30.7378 7.00721L30.2021 9.50002L32.864 9.59843L31.8161 11.916L34.4444 12.5569L33.0557 14.7L35.4148 15.7648L33.6218 17.5507Z" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path class="stroke-primary-200 fill-transparent group-enabled:group-hover:stroke-primary-100" d="M10.094 19.2668C10.094 6.98941 26.2081 6.66046 26.2081 18.9379M26.2081 18.9379L23.4875 16.5044M26.2081 18.9379L28.5101 15.8905" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                                    <rect class="stroke-primary-200 fill-primary-200 group-enabled:group-hover:stroke-primary-100 group-enabled:group-hover:fill-primary-100" x="17" y="17" width="4" height="4" rx="2"/>
                                </svg>
                            </button>
                            <button class="hover:stroke-primary-100 disabled:opacity-30 duration-300 group " title="Execute Next Instruction" onclick={on_execute_clicked} disabled={!datapath_state.initialized}>
                                <svg width="38" height="38" viewBox="0 0 38 38" xmlns="http://www.w3.org/2000/svg">
                                    <path class="stroke-primary-200 fill-transparent group-enabled:group-hover:stroke-primary-100" fill-rule="evenodd" clip-rule="evenodd" d="M34.4346 19.1007C34.4346 23.0891 32.8502 26.9142 30.0299 29.7345C27.2096 32.5547 23.3845 34.1392 19.3961 34.1392C15.4076 34.1392 11.5825 32.5547 8.76227 29.7345C5.94201 26.9142 4.3576 23.0891 4.3576 19.1007C4.3576 15.1122 5.94201 11.2871 8.76227 8.46686C11.5825 5.6466 15.4076 4.06219 19.3961 4.06219C23.3845 4.06219 27.2096 5.6466 30.0299 8.46686C32.8502 11.2871 34.4346 15.1122 34.4346 19.1007ZM34.3735 17.5507L36.4968 19.1007L34.3735 20.4552L36.1665 22.4366L33.8074 23.3059L35.1961 25.6444L32.6921 25.9915L33.6158 28.6029L30.8561 28.8385L31.4895 31.1941L28.8006 30.894L28.8983 33.3203L26.3846 32.508L25.9398 34.9007L23.699 33.6234L22.732 35.8711L20.8484 34.0981L19.3961 36.2013L17.9438 34.0981L16.0602 35.8711L15.0931 33.532L12.8523 34.9007L12.4076 32.4167L9.89384 33.3203L9.79543 30.8617L7.30262 31.1941L7.73998 28.8063L5.17641 28.6029L6.12595 26.3903L3.59607 25.6444L5.01061 23.7047L2.62563 22.4366L4.44452 20.4552L2.29541 19.1007L4.44452 17.5507L2.62563 15.7648L5.01061 14.7L3.59607 12.5569L6.12595 12.0144L5.17641 9.59843H7.73998L7.30262 7.00721L9.90477 7.44457L9.89384 4.881L12.3208 5.83053L12.8523 3.30066L15.0063 4.7152L16.0602 2.33022L17.857 4.14911L19.3961 2L20.7115 3.97315L22.732 2.33022L23.6712 4.30201L25.9398 3.30066L26.4823 5.83053L28.8983 4.881V7.44457L31.4895 7.00721L30.9538 9.50002L33.6158 9.59843L32.5678 11.916L35.1961 12.5569L33.8074 14.7L36.1665 15.7648L34.3735 17.5507Z" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path class="stroke-primary-200 fill-transparent group-enabled:group-hover:stroke-primary-100" d="M14.9766 13.536C14.9766 13.1414 15.4121 12.9023 15.745 13.1142L24.4895 18.6789C24.7983 18.8753 24.7983 19.326 24.4895 19.5225L15.745 25.0872C15.4121 25.2991 14.9766 25.0599 14.9766 24.6654V13.536Z" stroke-width="3"/>
                                    <path class="stroke-primary-200 fill-transparent group-enabled:group-hover:stroke-primary-100" d="M25.9731 11.8658L25.9731 26.3356" stroke-width="3" stroke-linecap="round"/>
                                </svg>
                            </button>
                            <button class="hover:stroke-primary-100 disabled:opacity-30 duration-300 group " title="Reset Program" onclick={on_reset_clicked} disabled={!datapath_state.initialized}>
                                <svg width="38" height="38" viewBox="0 0 38 38" xmlns="http://www.w3.org/2000/svg">
                                    <path class="stroke-primary-200 fill-transparent group-enabled:group-hover:stroke-primary-100" fill-rule="evenodd" clip-rule="evenodd" d="M34.1392 19.1007C34.1392 23.0891 32.5548 26.9142 29.7345 29.7345C26.9142 32.5548 23.0891 34.1392 19.1007 34.1392C15.1122 34.1392 11.2871 32.5548 8.46686 29.7345C5.6466 26.9142 4.06219 23.0891 4.06219 19.1007C4.06219 15.1122 5.6466 11.2871 8.46686 8.46686C11.2871 5.6466 15.1122 4.06219 19.1007 4.06219C23.0891 4.06219 26.9142 5.6466 29.7345 8.46686C32.5548 11.2871 34.1392 15.1122 34.1392 19.1007ZM34.0781 17.5507L36.2013 19.1007L34.0781 20.4552L35.8711 22.4366L33.512 23.3059L34.9007 25.6444L32.3966 25.9915L33.3203 28.6029L30.5606 28.8385L31.1941 31.1941L28.5052 30.894L28.6029 33.3203L26.0892 32.508L25.6444 34.9007L23.4036 33.6234L22.4366 35.8711L20.553 34.0981L19.1007 36.2013L17.6484 34.0981L15.7648 35.8711L14.7977 33.532L12.5569 34.9007L12.1121 32.4167L9.59843 33.3203L9.50002 30.8617L7.00721 31.1941L7.44457 28.8063L4.881 28.6029L5.83054 26.3903L3.30066 25.6444L4.7152 23.7047L2.33022 22.4366L4.14911 20.4552L2 19.1007L4.14911 17.5507L2.33022 15.7648L4.7152 14.7L3.30066 12.5569L5.83054 12.0144L4.881 9.59843H7.44457L7.00721 7.00721L9.60936 7.44457L9.59843 4.881L12.0254 5.83053L12.5569 3.30066L14.7109 4.7152L15.7648 2.33022L17.5616 4.14911L19.1007 2L20.4161 3.97315L22.4366 2.33022L23.3758 4.30201L25.6444 3.30066L26.1869 5.83053L28.6029 4.881V7.44457L31.1941 7.00721L30.6584 9.50002L33.3203 9.59843L32.2724 11.916L34.9007 12.5569L33.512 14.7L35.8711 15.7648L34.0781 17.5507Z" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path class="stroke-primary-200 fill-transparent group-enabled:group-hover:stroke-primary-100" d="M12.5 16.0526C13.3205 14.3684 15.1026 11 19.2051 11C23.3077 11 27 13.9474 27 19C27 24.0526 23.3077 27 19.2051 27C19.2051 27 19.2051 27 19.2051 27C19.2051 27 16.3333 27 14.6923 25.7368M12.5 16.0526C12.859 16.0526 17 16.0526 17 16.0526M12.5 16.0526C11.8591 14.9016 11 13.1053 11 13.1053" stroke-width="3" stroke-linecap="round"/>
                                </svg>
                            </button>
                        </div>
                    </div>

                    // Editor
                    <div class="flex flex-col grow min-h-16 mt-2">
                        <SwimEditor text_model={text_model} lines_content={lines_content} program_info={program_info_ref.borrow().clone()} pc_limit={*pc_limit} binary={binary_ref.borrow().clone()} memory_curr_instr={memory_curr_instr.clone()} editor_curr_line={editor_curr_line.clone()} editor_active_tab={editor_active_tab.clone()} console_active_tab={console_active_tab.clone()} pc={datapath_state.mips.registers.pc} communicator={props.communicator} current_architecture={datapath_state.current_architecture.clone()} speed={datapath_state.speed} sp={datapath_state.mips.registers[GpRegisterType::Sp]} memory={datapath_state.mips.memory.clone()}/>
                    </div>

                    // Console
                    <Footer parsermsg={(*parser_text_output).clone()} datapath_state={datapath_state.clone()} memory_text_model={memory_text_model} memory_curr_instr={memory_curr_instr.clone()} active_tab={console_active_tab.clone()} communicator={props.communicator} show_input={show_input.clone()} command={command.clone()} on_memory_clicked={on_memory_clicked.clone()} pc_limit={pc_limit.clone()}/>
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
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let file_input_elem = document
        .get_element_by_id("file_input")
        .expect("File input element with id \"file_input\" should exist.");

    let file_input_elem = file_input_elem
        .dyn_into::<HtmlInputElement>()
        .expect("Element should be an HtmlInputElement");

    // workaround for https://github.com/yewstack/yew/pull/3037 since it's not in 0.20
    spawn_local(async move {
        file_input_elem.click();
    });
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
