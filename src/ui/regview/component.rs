use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::emulation_core::mips::fp_registers::FpRegisters;
use crate::emulation_core::mips::gp_registers::GpRegisters;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::{html, Html};

#[derive(PartialEq, Properties)]
pub struct Regviewprops {
    pub gp: GpRegisters,
    pub fp: FpRegisters,
    pub pc_limit: usize,
    pub communicator: &'static DatapathCommunicator,
}
#[derive(PartialEq, Properties)]
pub struct Viewswitch {
    pub switch_view: bool,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum UnitState {
    #[default]
    Dec,
    Hex,
    Bin,
    Float,
    Double,
}
pub struct InputData {
    pub value: String,
    pub event: InputEvent,
}

//Convert register to html through iterator
// ============= General Purpose Registers =============
pub fn generate_gpr_rows(props: &Regviewprops, radix: u32) -> Html {
    let communicator = props.communicator;
    let pc_limit = props.pc_limit;

    props
        .gp
        .into_iter()
        .map(|(register, data)| {
            let format_string = match radix {
                16 => format!("{:#04x?}", data),
                2 => format!("{:#b}", data),
                _ => data.to_string(),
            };
            html! {
                <tr>
                    <td>{register.get_gpr_name()}</td>
                    <td>
                        <input type="text" id={register.to_string()}
                        oninput={move |e: InputEvent| {
                            let target = e.target();
                            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
                            let input_string = input.value();
                            let val = match u64::from_str_radix(&input_string[2..], radix) {
                                Ok(value) => {
                                    input.set_class_name("valid");
                                    value
                                },
                                Err(_err) => {
                                    input.set_class_name("invalid");
                                    return
                                }
                            };
                            if register.is_valid_register_value(val, pc_limit) {
                                communicator.set_register(register.to_string(), val);
                                input.set_class_name("valid");
                            } else {
                                input.set_class_name("invalid");
                            }
                        }}
                        value={format_string}/>
                    </td>
                </tr>
            }
        })
        .collect::<Html>()
}

// ============= Coprocessor Registers =============
pub fn generate_fpr_rows(props: &Regviewprops, unit_type: UnitState) -> Html {
    let communicator = props.communicator;

    props
        .fp
        .into_iter()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{format!("{register}")}</td>
                    <td>
                        <input type="text" id={register.to_string()}
                        oninput={move |e: InputEvent| {
                            let target = e.target();
                            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
                            let input_string = input.value();
                            let value = match unit_type {
                                UnitState::Float => {
                                    match input_string.parse::<f32>() {
                                        Ok(value) => {
                                            input.set_class_name("valid");
                                            value as u64
                                        },
                                        Err(_err) => {
                                            input.set_class_name("invalid");
                                            return
                                        }
                                    }
                                },
                                UnitState::Double => {
                                    match input_string.parse::<f64>() {
                                        Ok(value) => {
                                            input.set_class_name("valid");
                                            value as u64
                                        },
                                        Err(_err) => {
                                            input.set_class_name("invalid");
                                            return
                                        }
                                    }
                                },
                                UnitState::Hex => {
                                    match u64::from_str_radix(&input_string[2..], 16) {
                                        Ok(value) => {
                                            input.set_class_name("valid");
                                            value
                                        },
                                        Err(_err) => {
                                            input.set_class_name("invalid");
                                            return
                                        }
                                    }
                                },
                                UnitState::Bin => {
                                    match u64::from_str_radix(&input_string[2..], 2) {
                                        Ok(value) => {
                                            input.set_class_name("valid");
                                            value
                                        },
                                        Err(_err) => {
                                            input.set_class_name("invalid");
                                            return
                                        }
                                    }
                                },
                                _ => {
                                    match input_string.parse::<u64>() {
                                        Ok(value) => {
                                            input.set_class_name("valid");
                                            value
                                        },
                                        Err(_err) => {
                                            input.set_class_name("invalid");
                                            return
                                        }
                                    }
                                }
                            };
                            communicator.set_fp_register(register.to_string(), value);
                            if register.is_valid_register_value(value) {
                                input.set_class_name("valid");
                            } else {
                                input.set_class_name("invalid");
                            }
                        }}
                        value={
                            match unit_type {
                                UnitState::Float => format!("{:e}", data).to_string(),
                                UnitState::Double => format!("{:e}", data).to_string(),
                                UnitState::Hex => format!("{:#04x?}", data).to_string(),
                                UnitState::Bin => format!("{:#b}", data).to_string(),
                                _ => format!("{:?}", data).to_string(),
                            }
                        }/>
                    </td>
                </tr>
            }
        })
        .collect::<Html>()
}

#[function_component(Regview)]
pub fn regview(props: &Regviewprops) -> Html {
    let active_view = use_state_eq(UnitState::default);
    let switch_flag = use_state_eq(|| true);

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
                        UnitState::Double => "Double"
                    }
                }>
                    <option value="hex">{"Hex"}</option>
                    <option value="bin">{"Binary"}</option>
                    <option value="dec">{"Decimal"}</option>
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
                                {generate_gpr_rows(props, 2)}
                            }
                            else if *active_view == UnitState::Hex {
                                {generate_gpr_rows(props, 16)}
                            } else {
                                {generate_gpr_rows(props, 10)}
                            }
                        } else {
                            {generate_fpr_rows(props, *active_view.clone())}
                        }
                    </tbody>
                </table>
            </div>
        </div>
    }
}
