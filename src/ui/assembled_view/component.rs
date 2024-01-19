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
pub struct AssembledProps {
    pub text_model: TextModel,
    pub program_info: ProgramInfo,
    pub lines_content: Rc<RefCell<Vec<String>>>
}

#[function_component]
pub fn AssembledView(props: &AssembledProps) -> Html {
    let program_info = &props.program_info;
    let lines_content = props.lines_content.borrow_mut().clone();

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
        
    });

    let on_assembled_click = Callback::from(move |args: (MouseEvent, i32)| {
        let (e, line_number) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();

        debug!("Go to line number {:08x}", line_number);
        
    });

    let mut address = -4;
    html! {
        <table class={classes!("text_segment")}>
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