use std::cell::RefCell;
use std::rc::Rc;

use monaco::api::TextModel;
use web_sys::HtmlInputElement;
use yew::{Properties, Html};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use log::debug;
use crate::parser::parser_structs_and_enums::ProgramInfo;


#[derive(PartialEq, Properties)]
pub struct TextSegmentProps {
    pub program_info: ProgramInfo,
    pub lines_content: Rc<RefCell<Vec<String>>>,
    pub memory_curr_line: Rc<RefCell<f64>>
}
#[derive(PartialEq, Properties)]
pub struct DataSegmentProps {
    pub program_info: ProgramInfo,
    pub binary: Vec<u32>,
    pub lines_content: Rc<RefCell<Vec<String>>>,
    pub memory_curr_line: Rc<RefCell<f64>>
}

#[function_component]
pub fn TextSegment(props: &TextSegmentProps) -> Html {
    let program_info = &props.program_info;
    let lines_content = props.lines_content.borrow_mut().clone();
    let memory_curr_line = props.memory_curr_line.borrow_mut();

    let on_check = Callback::from(move |args: (MouseEvent, i32)| {
        let (e, address) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();

        if (input.checked()) {
            debug!("Breakpoint set at {:08x}", address);
        }
        
    });

    let on_address_click = Callback::from(move |args: (MouseEvent, i32)| {
        let (e, address) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();

        debug!("Go to address {:08x}", address);
        debug!("Go to line {:?}", (address / 4) as f64);
        // *memory_curr_line = (address / 4) as f64;
        
    });

    let on_assembled_click = Callback::from(move |args: (MouseEvent, i32)| {
        let (e, line_number) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();

        debug!("Go to line number {:08x}", line_number);
        
    });

    let mut address = -4;
    html! {
        <table class={classes!("memory_segment")}>
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
                    address += 4;
                    html!{ 
                        
                        <tr key={index} class={classes!("row")}>
                            <td class={classes!("bkpt")}>
                                <input type="checkbox" onclick={move |e: MouseEvent| {on_check.emit((e, address))}}/>
                                <div class="circle"></div>
                            </td>
                            <td class="address" title={format!("Go to address {:08x}", address)} onclick={move |e: MouseEvent| {on_address_click.emit((e, address))}}>
                                {format!("0x{:08x}", address as u64)} 
                            </td>
                            <td>
                                {format!("0b{:032b}", instruction.binary)} 
                            </td>
                            <td>
                                {format!("0x{:08x}", instruction.binary)} 
                            </td>
                            <td class="assembled-string" title="Go to line" onclick={move |e: MouseEvent| {on_assembled_click.emit((e, address))}}>
                                {recreated_string} 
                            </td>
                            <td>
                                {format!("{}: {:?}", instruction.line_number, lines_content.get(instruction.line_number).unwrap_or(&String::from("")))}
                            </td>
                        </tr>
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
    let memory_curr_line = props.memory_curr_line.borrow_mut();

    let on_address_click = Callback::from(move |args: (MouseEvent, usize)| {
        let (e, address) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();

        debug!("Go to address {:08x}", address);
        debug!("Go to line {:?}", (address / 4) as f64);
        // *memory_curr_line = (address / 4) as f64;
        
    });

    let on_assembled_click = Callback::from(move |args: (MouseEvent, usize)| {
        let (e, line_number) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();

        debug!("Go to line number {:08x}", line_number);
        
    });

    html! {
        <table class={classes!("memory_segment")}>
        // | address | data in hex | source string
            <tr>
                <th>{"Address"}</th>
                <th>{"Hex"}</th>
                <th>{"Assembled"}</th>
                <th>{"Source"}</th>
            </tr>
            { 
                if program_info.instructions.len() > 0 {
                    let mut address = program_info.instructions.len() * 4 - 4;
                    program_info.data.iter().enumerate().map(|(index, data)| {
                        let recreated_string = data.recreate_string();
                        let on_address_click = Callback::clone(&on_address_click);
                        let on_assembled_click = Callback::clone(&on_assembled_click);
                        address += 4;
                        html!{ 
                            
                            <tr key={index} class={classes!("row")}>
                                <td class="address" title={format!("Go to address {:08x}", address)} onclick={move |e: MouseEvent| {on_address_click.emit((e, address))}}>
                                    {format!("0x{:08x}", address as u64)} 
                                </td>
                                <td>
                                    {format!("0x{:08x}", binary[data.line_number])} 
                                </td>
                                <td class="assembled-string" title="Go to line" onclick={move |e: MouseEvent| {on_assembled_click.emit((e, address))}}>
                                    {recreated_string} 
                                </td>
                                <td>
                                    {format!("{}: {:?}", data.line_number, lines_content.get(data.line_number).unwrap_or(&String::from("")))}
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