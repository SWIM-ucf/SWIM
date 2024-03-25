use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::emulation_core::mips::memory::{Memory, MemoryIter};
use crate::emulation_core::mips::stack::Stack;
// use monaco::api::TextModel;
use crate::parser::parser_structs_and_enums::ProgramInfo;
use crate::ui::swim_editor::tab::TabState;
use log::debug;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew::{Html, Properties};

// TODO: Create Segment Viewer component for extendability to any segment

#[derive(PartialEq, Properties)]
pub struct TextSegmentProps {
    pub program_info: ProgramInfo,
    pub lines_content: Rc<RefCell<Vec<String>>>,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub editor_curr_line: UseStateHandle<f64>,
    pub pc: u64,
    pub editor_active_tab: UseStateHandle<TabState>,
    pub console_active_tab: UseStateHandle<TabState>,
    pub communicator: &'static DatapathCommunicator,
}
#[derive(PartialEq, Properties)]
pub struct DataSegmentProps {
    pub program_info: ProgramInfo,
    pub binary: Vec<u32>,
    pub lines_content: Rc<RefCell<Vec<String>>>,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub editor_curr_line: UseStateHandle<f64>,
    pub editor_active_tab: UseStateHandle<TabState>,
    pub console_active_tab: UseStateHandle<TabState>,
    pub pc_limit: usize,
}

#[function_component]
pub fn TextSegment(props: &TextSegmentProps) -> Html {
    let program_info = &props.program_info;
    let lines_content = props.lines_content.borrow_mut().clone();
    let memory_curr_instr = &props.memory_curr_instr;
    let editor_curr_line = &props.editor_curr_line;
    let editor_active_tab = &props.editor_active_tab;
    let console_active_tab = &props.console_active_tab;
    let executed_ref = use_node_ref();
    let communicator = props.communicator;

    {
        // Always scroll to the executed row on execution
        let executed_row = executed_ref.cast::<HtmlElement>();
        if let Some(executed_row) = executed_row {
            let mut options = web_sys::ScrollIntoViewOptions::new();
            options.block(web_sys::ScrollLogicalPosition::Center);
            executed_row.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }

    let on_check = Callback::from(move |args: (MouseEvent, i64)| {
        let (e, address) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();

        if input.checked() {
            debug!("Breakpoint set at {:08x}", address as u64);
            communicator.set_breakpoint(address as u64);
        }
    });

    // Go to the memory address in hex editor
    let on_address_click = {
        let memory_curr_instr = memory_curr_instr.clone();
        let console_active_tab = console_active_tab.clone();
        use_callback(
            move |args: (MouseEvent, usize), memory_curr_instr| {
                let (_e, address) = args;
                memory_curr_instr.set(address as u64);
                console_active_tab.set(TabState::HexEditor);
            },
            memory_curr_instr,
        )
    };

    // Go to the line in code editor
    let on_assembled_click = {
        let editor_curr_line = editor_curr_line.clone();
        let editor_active_tab = editor_active_tab.clone();
        use_callback(
            move |args: (MouseEvent, usize), _| {
                let (_e, line_number) = args;
                editor_curr_line.set(line_number as f64 + 1.0);
                editor_active_tab.set(TabState::Editor);
            },
            (),
        )
    };

    let mut address = -4;
    html! {
        <table class="h-[96%] bg-primary-900">
        // | breakpoint checkbox | address | instruction in binary | instruction in hex | updated string | source string
            <tr>
                <th>{"Bkpt"}</th>
                <th>{"Address"}</th>
                <th>{"Binary"}</th>
                <th>{"Hex"}</th>
                <th>{"Assembled"}</th>
                <th>{"Source"}</th>
            </tr>
            {
                program_info.instructions.iter().enumerate().map(|(index, instruction)| {
                    let recreated_string = instruction.recreate_string();
                    let on_check = Callback::clone(&on_check);
                    let on_address_click = Callback::clone(&on_address_click);
                    let on_assembled_click = Callback::clone(&on_assembled_click);
                    let executed_ref = executed_ref.clone();
                    address += 4;

                    let line_number = instruction.line_number;

                    let mut conditional_class = "";
                    if props.pc as i64 == address + 4 {
                        conditional_class = "bg-primary-700 shadow-executing";
                        html!{
                            <tr ref={executed_ref} key={index} class={classes!(conditional_class)}>
                                <td class="h-full relative group">
                                    <input type="checkbox" class="hover:cursor-pointer peer absolute top-0 left-0 opacity-0 w-full h-full" onclick={move |e: MouseEvent| {on_check.emit((e, address))}}/>
                                    <div class="h-3 w-3 rounded-3xl m-auto bg-transparent group-hover:bg-accent-blue-200 peer-checked:bg-accent-blue-100"></div>
                                </td>
                                <td class="text-accent-green-300 hover:text-accent-green-200 cursor-pointer" title={format!("Go to address in memory {:08x}", address)} onclick={move |e: MouseEvent| {on_address_click.emit((e, address as usize))}}>
                                    {format!("0x{:08x}", address as u64)}
                                </td>
                                <td>
                                    {format!("0b{:032b}", instruction.binary)}
                                </td>
                                <td>
                                    {format!("0x{:08x}", instruction.binary)}
                                </td>
                                <td class="text-accent-blue-200 hover:text-accent-blue-100 cursor-pointer" title="Go to line in editor" onclick={move |e: MouseEvent| {on_assembled_click.emit((e, line_number))}}>
                                    {recreated_string}
                                </td>
                                <td>
                                    {format!("{}: {:?}", line_number, lines_content.get(line_number).unwrap_or(&String::from("")))}
                                </td>
                            </tr>
                        }
                    }
                    else {
                        html!{
                            <tr key={index} class={classes!(conditional_class)}>
                                <td class="h-full relative group">
                                    <input type="checkbox" class="hover:cursor-pointer peer absolute top-0 left-0 opacity-0 w-full h-full" onclick={move |e: MouseEvent| {on_check.emit((e, address))}}/>
                                    <div class="h-3 w-3 rounded-3xl m-auto bg-transparent group-hover:bg-accent-blue-200 peer-checked:bg-accent-blue-100"></div>
                                </td>
                                <td class="text-accent-green-300 hover:text-accent-green-200 cursor-pointer" title={format!("Go to address in memory {:08x}", address)} onclick={move |e: MouseEvent| {on_address_click.emit((e, address as usize))}}>
                                    {format!("0x{:08x}", address as u64)}
                                </td>
                                <td>
                                    {format!("0b{:032b}", instruction.binary)}
                                </td>
                                <td>
                                    {format!("0x{:08x}", instruction.binary)}
                                </td>
                                <td class="text-accent-blue-200 hover:text-accent-blue-100 cursor-pointer" title="Go to line in editor" onclick={move |e: MouseEvent| {on_assembled_click.emit((e, line_number))}}>
                                    {recreated_string}
                                </td>
                                <td>
                                    {format!("{}: {:?}", line_number + 1, lines_content.get(line_number).unwrap_or(&String::from("")))}
                                </td>
                            </tr>
                        }
                    }
                }).collect::<Html>()
            }
        </table>
    }
}

#[function_component]
pub fn DataSegment(props: &DataSegmentProps) -> Html {
    let program_info = &props.program_info;
    let binary = &props.binary;
    let lines_content = props.lines_content.borrow_mut().clone();
    let memory_curr_instr = &props.memory_curr_instr;
    let editor_curr_line = &props.editor_curr_line;
    let editor_active_tab = &props.editor_active_tab;
    let console_active_tab = &props.console_active_tab;

    // Go to the memory address in hex editor
    let on_address_click = {
        let memory_curr_instr = memory_curr_instr.clone();
        let console_active_tab = console_active_tab.clone();
        use_callback(
            move |args: (MouseEvent, usize), memory_curr_instr| {
                let (_e, address) = args;
                memory_curr_instr.set(address as u64);
                console_active_tab.set(TabState::HexEditor);
            },
            memory_curr_instr,
        )
    };

    // Go to the line in code editor
    let on_assembled_click = {
        let editor_curr_line = editor_curr_line.clone();
        let editor_active_tab = editor_active_tab.clone();
        use_callback(
            move |args: (MouseEvent, usize), _| {
                let (_e, line_number) = args;
                editor_curr_line.set(line_number as f64);
                editor_active_tab.set(TabState::Editor);
            },
            (),
        )
    };

    html! {
        <table class="h-[96%] bg-primary-900">
        // | address | data in hex | source string
            <tr>
                <th>{"Address"}</th>
                <th>{"Hex"}</th>
                <th>{"Assembled"}</th>
                <th>{"Source"}</th>
            </tr>
            {
                if !program_info.instructions.is_empty() && !binary.is_empty() {
                    let mut address = program_info.instructions.len() * 4 - 4;
                    let mut data_binary_index = program_info.data_starting_point - 1;
                    program_info.data.iter().enumerate().map(|(index, data)| {
                        let recreated_string = data.recreate_string();
                        let on_address_click = Callback::clone(&on_address_click);
                        let on_assembled_click = Callback::clone(&on_assembled_click);
                        address += 4;
                        data_binary_index += 1;
                        html!{

                            <tr key={index}>
                                <td class="text-accent-green-300 hover:text-accent-green-200 cursor-pointer" title={format!("Go to address in memory {:08x}", address)} onclick={move |e: MouseEvent| {on_address_click.emit((e, address))}}>
                                    {format!("0x{:08x}", address as u64)}
                                </td>
                                <td>
                                    {format!("0x{:08x}", binary[data_binary_index])}
                                </td>
                                <td class="text-accent-blue-200 hover:text-accent-blue-100 cursor-pointer" title="Go to line" onclick={move |e: MouseEvent| {on_assembled_click.emit((e, address))}}>
                                    {recreated_string}
                                </td>
                                <td>
                                    {format!("{}: {:?}", data.line_number + 1, lines_content.get(data.line_number).unwrap_or(&String::from("")))}
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
                else {
                    html! {<></>}
                }
            }
        </table>
    }
}

#[derive(PartialEq, Properties)]
pub struct StackSegmentProps {
    pub memory: Memory,
    pub sp: u64,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub console_active_tab: UseStateHandle<TabState>,
}

#[function_component]
pub fn StackSegment(props: &StackSegmentProps) -> Html {
    let memory = &props.memory;
    let sp = props.sp;
    let console_active_tab = &props.console_active_tab;
    let memory_curr_instr = &props.memory_curr_instr;

    // Go to the memory address in hex editor
    let on_address_click = {
        let memory_curr_instr = memory_curr_instr.clone();
        let console_active_tab = console_active_tab.clone();
        use_callback(
            move |args: (MouseEvent, usize), memory_curr_instr| {
                let (_e, address) = args;
                memory_curr_instr.set(address as u64);
                console_active_tab.set(TabState::HexEditor);
            },
            memory_curr_instr,
        )
    };

    html! {
        <table class="h-[96%] bg-primary-900">
        // | address | data in hex
            <tr>
                <th>{"Address"}</th>
                <th>{"Hex"}</th>
            </tr>
            {
                if !memory.memory.is_empty() && sp != 0 {
                    let memory_iter = MemoryIter::new(memory, sp as usize, memory.memory.len());
                    memory_iter.map(|(address, words)| {
                        let on_address_click = Callback::clone(&on_address_click);
                        html! {
                            <tr>
                                <td class="text-accent-green-300 hover:text-accent-green-200 cursor-pointer" title={format!("Go to address in memory {:08x}", address)} onclick={move |e: MouseEvent| {on_address_click.emit((e, address))}}>
                                    {format!("0x{:08x}", address as u64)}
                                </td>
                                <td>
                                    {
                                        words.iter().fold(String::new(), |acc, word| {
                                            format!("{}0x{:08x} ", acc, word)
                                        })
                                    }
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
                else {
                    html! {<></>}
                }
            }
        </table>
    }
}

#[derive(PartialEq, Properties)]
pub struct StackFrameProps {
    pub stack: Stack,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub console_active_tab: UseStateHandle<TabState>,
    pub program_info: ProgramInfo,
    pub labels: HashMap<String, usize>,
}

#[function_component]
pub fn StackFrameView(props: &StackFrameProps) -> Html {
    let console_active_tab = &props.console_active_tab;
    let memory_curr_instr = &props.memory_curr_instr;
    let stack = &props.stack;
    let program_info = &props.program_info;
    let labels = &props.labels;

    // Go to the memory address in hex editor
    let on_address_click = {
        let memory_curr_instr = memory_curr_instr.clone();
        let console_active_tab = console_active_tab.clone();
        use_callback(
            move |args: (MouseEvent, usize), memory_curr_instr| {
                let (_e, address) = args;
                memory_curr_instr.set(address as u64);
                console_active_tab.set(TabState::HexEditor);
            },
            memory_curr_instr,
        )
    };

    html! {
        <table class="h-[96%] bg-primary-900">
        // | address | data in hex
            <tr>
                <th>{"Label"}</th>
                <th>{"Call Address"}</th>
                <th>{"Call Line"}</th>
                <th>{"Return Address"}</th>
                <th>{"Frame Pointer"}</th>
                <th>{"Stack Pointer"}</th>
            </tr>
            {
                if !stack.stack.is_empty() && program_info.instructions.len() > 0 {
                    let stack = stack.stack.clone();
                    stack.into_iter().enumerate().map(|(_address, frame)| {
                        let call_line_number = frame.call_address / 4;
                        log::debug!("Call Line Number: {}", call_line_number);
                        let call_recreated_string = program_info.instructions[call_line_number as usize].recreate_string();
                        log::debug!("Call Recreated String: {:?}", call_recreated_string);
                        let on_call_address_click = Callback::clone(&on_address_click);
                        let on_return_address_click = Callback::clone(&on_address_click);
                        let on_stack_pointer_click = Callback::clone(&on_address_click);
                        log::debug!("Jump to address: {:?}", frame.jump_address);
                        let default_label = String::from("");
                        let label = labels.iter().find_map(|(label, address)| {
                            if *address == frame.jump_address as usize {
                                Some(label)
                            }
                            else {
                                None
                            }
                        }).unwrap_or(&default_label);
                        log::debug!("Frame: {:?}", frame);
                        html! {
                            <tr>
                                <td>
                                    {label}
                                </td>
                                <td class="text-accent-green-300 hover:text-accent-green-200 cursor-pointer" title={format!("Go to address in memory {:08x}", frame.call_address)} onclick={move |e: MouseEvent| {on_call_address_click.emit((e, frame.call_address as usize))}}>
                                    {format!("0x{:08x}", frame.call_address as u64)}
                                </td>
                                <td>
                                    {format!("{}: {:?}", call_line_number + 1, call_recreated_string)}
                                </td>
                                <td class="text-accent-green-300 hover:text-accent-green-200 cursor-pointer" title={format!("Go to address in memory {:08x}", frame.return_address)} onclick={move |e: MouseEvent| {on_return_address_click.emit((e, frame.return_address as usize))}}>
                                    {format!("0x{:08x}", frame.return_address)}
                                </td>
                                <td>
                                    {format!("{:?}", frame.frame_pointer)}
                                </td>
                                <td class="text-accent-green-300 hover:text-accent-green-200 cursor-pointer" title={format!("Go to address in memory {:08x}", frame.stack_pointer)} onclick={move |e: MouseEvent| {on_stack_pointer_click.emit((e, frame.stack_pointer as usize))}}>
                                    {format!("0x{:08x}", frame.stack_pointer)}
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
                else {
                    html! {<></>}
                }
            }
        </table>
    }
}
