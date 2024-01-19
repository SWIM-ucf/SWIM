use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::memory::CAPACITY_BYTES;
use crate::emulation_core::mips::registers::{GpRegisterType, GpRegisters};
//use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, InputEvent, HtmlInputElement};
use yew::prelude::*;
use yew::{html, Html};
use std::rc::Rc;
use std::cell::RefCell;
// use log::debug;

// datapath.coprocessor.fpr
#[derive(PartialEq, Properties)]
pub struct Regviewprops {
    pub gp: GpRegisters,
    pub fp: [u64; 32],
    pub datapath: Rc<RefCell<MipsDatapath>>,
    pub pc_limit: usize,
    // pub communicator: &'static DatapathCommunicator
}
#[derive(PartialEq, Properties)]
pub struct Regrowprops {
    pub gp: GpRegisters,
    pub fp: [u64; 32],
    pub on_input: Callback<(InputEvent, GpRegisterType)>
}
#[derive(PartialEq, Properties)]
pub struct Viewswitch {
    pub switch_view: bool,
}

#[derive(Default, PartialEq)]
enum UnitState {
    #[default]
    Dec,
    Hex,
    Bin,
    Float,
    Double,
}

// #[derive(Debug)]
// enum Msg {
//     UpdateRegister(GpRegisterType, i64),
// }

pub struct InputData {
    pub value: String,
    pub event: InputEvent,
}

//Convert register to html through iterator
pub fn generate_gpr_rows(props: &Regrowprops) -> Html {

    props.gp.into_iter()
        .map(|(register, data)| {
            let on_input = Callback::clone(&props.on_input);
            html! {
                <tr>
                    <td>{get_gpr_name(register)}</td>
                    <td>
                        <input type="text" id={register.to_string()}
                        oninput={move |e: InputEvent| {on_input.emit((e, register))}}
                        value={(data as i64).to_string()}/>
                    </td>
                </tr>
            }
        })
        .collect::<Html>()
}
pub fn generate_gpr_rows_hex(gp: GpRegisters) -> Html {
    gp.into_iter()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{get_gpr_name(register)}</td>
                    <td>{format!("{data:#04x?}").to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}
pub fn generate_gpr_rows_bin(gp: GpRegisters) -> Html {
    gp.into_iter()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{get_gpr_name(register)}</td>
                    <td>{format!("{data:#b}").to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}
pub fn generate_fpr_rows(fp: [u64; 32]) -> Html {
    fp.iter()
        .enumerate()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{format!("f{register}")}</td>
                    <td>{(*data as i64).to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}
pub fn generate_fpr_rows_hex(fp: [u64; 32]) -> Html {
    fp.iter()
        .enumerate()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{format!("f{register}")}</td>
                    <td>{format!("{data:#04x?}").to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}
pub fn generate_fpr_rows_bin(fp: [u64; 32]) -> Html {
    fp.iter()
        .enumerate()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{format!("f{register}")}</td>
                    <td>{format!("{data:#b}").to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}

pub fn generate_fpr_rows_float(fp: [u64; 32]) -> Html {
    fp.iter()
        .enumerate()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{format!("f{register}")}</td>
                    <td>{format!("{:e}",f32::from_bits((*data).try_into().unwrap())).to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}

pub fn generate_fpr_rows_double(fp: [u64; 32]) -> Html {
    fp.iter()
        .enumerate()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{format!("f{register}")}</td>
                    <td>{format!("{:e}", f64::from_bits(*data)).to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}

/// Returns the text to be shown for a general-purpose register.
pub fn get_gpr_name(register: GpRegisterType) -> String {
    if register == GpRegisterType::Pc {
        register.to_string()
    } else {
        format!("{} (r{})", register, register as u32)
    }
}

#[function_component(Regview)]
pub fn regview(props: &Regviewprops) -> Html {
    let active_view = use_state_eq(UnitState::default);
    let switch_flag = use_state_eq(|| true);

    let datapath = Rc::clone(&props.datapath);
    let pc_limit = props.pc_limit;
    let change_view = {
        let active_view = active_view.clone();
        Callback::from(move |event: Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            let mode = target.value();

            let new_mode = match mode.as_str() {
                "bin" => UnitState::Bin,
                "hex" => UnitState::Hex,
                "dec" => UnitState::Dec,
                "float" => UnitState::Float,
                "double" => UnitState::Double,
                _ => UnitState::default(),
            };

            active_view.set(new_mode);
        })
    };
    let on_switch_clicked_fp = {
        let switch_flag = switch_flag.clone();
        use_callback(
            move |_, switch_flag| {
                if **switch_flag {
                    switch_flag.set(false);
                }
            },
            switch_flag,
        )
    };
    let on_switch_clicked_gp = {
        let switch_flag = switch_flag.clone();
        use_callback(
            move |_, switch_flag| {
                if !(**switch_flag) {
                    switch_flag.set(true);
                }
            },
            switch_flag,
        )
    };

    let on_input = Callback::from(move |args: (InputEvent, GpRegisterType)| {
        let (e, register) = args;
        let target = e.target();
        let input = target.unwrap().unchecked_into::<HtmlInputElement>();
        let val: i64 = match input.value().parse() {
            Ok(value) => {
                input.style().set_property("color", "black").unwrap_or_default();
                value
            },
            Err(_err) => {
                input.style().set_property("color", "red").unwrap_or_default();
                return
            }
        };
        // let msg = Msg::UpdateRegister(register, val);

        let mut datapath = datapath.borrow_mut();

        let write_destination: usize = register as usize;
        if register == GpRegisterType::Pc {
            // check if pc is more than the number of instructions
            // or if it's not word aligned
            if val > pc_limit as i64 || val % 4 != 0
            {
                input.style().set_property("color", "red").unwrap_or_default();
                return
            }

            datapath.registers.pc = val as u64;
            // props.communicator.send_test_message(val);
        }
        // check if pc is more than memory capacity
        // or if it's not word aligned
        else if register == GpRegisterType::Sp {
            if val > CAPACITY_BYTES as i64 || val < 0 || val % 4 != 0 {
                input.style().set_property("color", "red").unwrap_or_default();
                return
            }
        }
        else {
            datapath.registers.gpr[write_destination] = val as u64;
        }
    });

    let rowprops = Regrowprops {
        gp: props.gp,
        fp: props.fp,
        on_input
    };

    //log!("This is ", *switch_flag);
    html! {
        <div style="flex-grow: 1; display: flex; flex-direction: column; flex-wrap: nowrap; margin-top: 36px;">
            <div class="regview-menu bar">
                <div class="tabs">
                    if *switch_flag {
                        <button class={classes!("tab", "pressed")} onclick={on_switch_clicked_gp.clone()}>{"GP"}</button>
                    } else {
                        <button class="tab" onclick={on_switch_clicked_gp.clone()}>{"GP"}</button>
                    }
                    if !(*switch_flag){
                        <button class={classes!("tab", "pressed")} onclick={on_switch_clicked_fp.clone()}>{"FP"}</button>
                    } else {
                        <button class="tab" onclick={on_switch_clicked_fp.clone()}>{"FP"}</button>
                    }
                </div>
                <select class="unit-state" name="units" onchange={change_view.clone()} value={
                    match *active_view {
                        UnitState::Bin => "Binary",
                        UnitState::Dec => "Decimal",
                        UnitState::Hex => "Hex",
                        UnitState::Float => "Float",
                        UnitState::Double => "Double",
                        _ => "dec",
                    }
                }>
                    <option value="dec">{"Decimal"}</option>
                    <option value="bin">{"Binary"}</option>
                    <option value="hex">{"Hex"}</option>
                    if !*switch_flag {
                        <option value="float">{"Float"}</option>
                        <option value="double">{"Double"}</option>
                    }
                </select>
            </div>
            <div class="table-wrapper">
                <table style="background-color: #ffffff">
                    <thead>
                        <tr>
                            <th>{"Register Name"}</th>
                            <th>{"Data"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        if *switch_flag{
                            if *active_view == UnitState::Bin {
                                {generate_gpr_rows_bin(props.gp)}
                            }
                            else if *active_view == UnitState::Hex {
                                {generate_gpr_rows_hex(props.gp)}
                            } else {
                                {generate_gpr_rows(&rowprops)}
                            }
                        } else {
                            if *active_view == UnitState::Bin {
                                {generate_fpr_rows_bin(props.fp)}
                            } else if *active_view == UnitState::Hex{
                                {generate_fpr_rows_hex(props.fp)}
                            } else if *active_view == UnitState::Float{
                                {generate_fpr_rows_float(props.fp)}
                            } else if *active_view == UnitState::Double{
                                {generate_fpr_rows_double(props.fp)}
                            } else if *active_view == UnitState::Dec {
                                {generate_fpr_rows(props.fp)}
                            }
                        }
                    </tbody>
                </table>
            </div>
        </div>
    }
}
