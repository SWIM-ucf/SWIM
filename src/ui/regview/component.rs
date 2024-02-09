use crate::agent::datapath_communicator::DatapathCommunicator;
use crate::emulation_core::mips::registers::GpRegisters;
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use yew::prelude::*;
use yew::{html, Html};

#[derive(PartialEq, Properties)]
pub struct Regviewprops {
    pub gp: GpRegisters,
    pub fp: [u64; 32],
    pub pc_limit: usize,
    pub communicator: &'static DatapathCommunicator
}
#[derive(PartialEq, Properties)]
pub struct Regrowprops {
    pub gp: GpRegisters,
    pub fp: [u64; 32],
    pub pc_limit: usize,
    pub communicator: &'static DatapathCommunicator
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
pub struct InputData {
    pub value: String,
    pub event: InputEvent,
}

//Convert register to html through iterator
pub fn generate_gpr_rows(props: &Regrowprops) -> Html {
    let communicator = props.communicator;
    let pc_limit = props.pc_limit;

    props.gp.into_iter()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{register.get_gpr_name()}</td>
                    <td>
                        <input type="text" id={register.to_string()}
                        oninput={move |e: InputEvent| {
                            let target = e.target();
                            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
                            let val: u64 = match input.value().parse() {
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
                    <td>{register.get_gpr_name()}</td>
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
                    <td>{register.get_gpr_name()}</td>
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

    let rowprops = Regrowprops {
        gp: props.gp,
        fp: props.fp,
        pc_limit: props.pc_limit,
        communicator: props.communicator
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
