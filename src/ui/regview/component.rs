use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::emulation_core::register::RegisterType;
use crate::ui::swim_editor::tab::Tab;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew::{html, Html};

#[derive(PartialEq, Properties)]
pub struct Regviewprops {
    pub gp: Vec<(Rc<dyn RegisterType>, u64)>,
    pub fp: Vec<(Rc<dyn RegisterType>, u64)>,
    pub pc_limit: usize,
    pub communicator: &'static DatapathCommunicator,
}
#[derive(PartialEq, Default)]
pub enum RegviewTabState {
    #[default]
    Gp,
    Fp,
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
    let registers = props.gp.clone();

    registers
        .into_iter()
        .map(|(register, data)| {
            let format_string = match radix {
                16 => format!("{:#04x?}", data),
                2 => format!("{:#b}", data),
                _ => data.to_string(),
            };
            html! {
                <tr>
                    <td>{register.get_register_name()}</td>
                    <td>
                        <input type="text" id={register.to_string()}
                        oninput={move |e: InputEvent| {
                            let target = e.target();
                            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
                            let input_string = input.value();
                            let mut number = input_string.as_str();
                            if radix == 2 || radix == 16 {
                                number = &input_string[2..];
                            }
                            let val = match u64::from_str_radix(number, radix) {
                                Ok(value) => {
                                    input.set_class_name("");
                                    value
                                },
                                Err(_err) => {
                                    input.set_class_name("text-accent-red-200");
                                    return
                                }
                            };
                            if register.is_valid_register_value(val, pc_limit) {
                                communicator.set_register(register.to_string(), val);
                                input.set_class_name("");
                            } else {
                                input.set_class_name("text-accent-red-200");
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
    let pc_limit = props.pc_limit;
    let registers = props.fp.clone();

    registers
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
                                            input.set_class_name("");
                                            value.to_bits() as u64
                                        },
                                        Err(_err) => {
                                            input.set_class_name("text-accent-red-200");
                                            return
                                        }
                                    }
                                },
                                UnitState::Double => {
                                    match input_string.parse::<f64>() {
                                        Ok(value) => {
                                            input.set_class_name("");
                                            value.to_bits() as u64
                                        },
                                        Err(_err) => {
                                            input.set_class_name("text-accent-red-200");
                                            return
                                        }
                                    }
                                },
                                UnitState::Hex => {
                                    match u64::from_str_radix(&input_string[2..], 16) {
                                        Ok(value) => {
                                            input.set_class_name("");
                                            value
                                        },
                                        Err(_err) => {
                                            input.set_class_name("text-accent-red-200");
                                            return
                                        }
                                    }
                                },
                                UnitState::Bin => {
                                    match u64::from_str_radix(&input_string[2..], 2) {
                                        Ok(value) => {
                                            input.set_class_name("");
                                            value
                                        },
                                        Err(_err) => {
                                            input.set_class_name("text-accent-red-200");
                                            return
                                        }
                                    }
                                },
                                _ => {
                                    match input_string.parse::<u64>() {
                                        Ok(value) => {
                                            input.set_class_name("");
                                            value
                                        },
                                        Err(_err) => {
                                            input.set_class_name("text-accent-red-200");
                                            return
                                        }
                                    }
                                }
                            };
                            if register.is_valid_register_value(value, pc_limit) {
                                communicator.set_fp_register(register.to_string(), value);
                                input.set_class_name("");
                            } else {
                                input.set_class_name("text-accent-red-200");
                            }
                        }}
                        value={
                            match unit_type {
                                UnitState::Float => format!("{:e}", f32::from_bits(data as u32)).to_string(),
                                UnitState::Double => format!("{:e}", f64::from_bits(data)).to_string(),
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
    let active_tab = use_state_eq(RegviewTabState::default);

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
    let change_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();
            let tab_name = target.get_attribute("label").unwrap_or(String::from("gp"));

            let new_tab = match tab_name.as_str() {
                "gp" => RegviewTabState::Gp,
                "fp" => RegviewTabState::Fp,
                _ => RegviewTabState::default(),
            };

            active_tab.set(new_tab);
        })
    };

    html! {
        <div class="grow flex flex-col flex-no-wrap mt-12 min-w-0">
            <div class="flex flex-row justify-between">
                <div>
                    <Tab<RegviewTabState> label="gp" text="GP" on_click={change_tab.clone()} disabled={false} active_tab={active_tab.clone()} tab_name={RegviewTabState::Gp}/>
                    <Tab<RegviewTabState> label="fp" text="FP" on_click={change_tab.clone()} disabled={false} active_tab={active_tab.clone()} tab_name={RegviewTabState::Fp}/>
                </div>
                <select class="text-right bg-primary-600 text-primary-200 flex items-center flex-row" name="units" onchange={change_view.clone()} value={
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
                    if *active_tab == RegviewTabState::Fp {
                        <option value="float">{"Float"}</option>
                        <option value="double">{"Double"}</option>
                    }
                </select>
            </div>
            <div class="overflow-y-auto">
                <table>
                    <thead>
                        <tr>
                            <th class="bg-primary-800">{"Register Name"}</th>
                            <th class="bg-primary-800">{"Data"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        if *active_tab == RegviewTabState::Gp {
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
