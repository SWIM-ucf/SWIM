use crate::emulation_core::mips::registers::GpRegisters;
//use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

// datapath.coprocessor.fpr
#[derive(PartialEq, Properties)]
pub struct Regviewprops {
    pub gp: GpRegisters,
    pub fp: [u64; 32],
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

//Convert register to html through iterator
pub fn generate_gpr_rows(gp: GpRegisters) -> Html {
    gp.into_iter()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{register}</td>
                    <td>{(data as i64).to_string()}</td>
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
                    <td>{register}</td>
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
                    <td>{register}</td>
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
                    <td>{format!("F{register}")}</td>
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
                    <td>{format!("F{register}")}</td>
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
                    <td>{format!("F{register}")}</td>
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
                    <td>{format!("F{register}")}</td>
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
                    <td>{format!("F{register}")}</td>
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
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            let mode = target
                .get_attribute("label")
                .unwrap_or(String::from("regview"));

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
        <div style="flex-grow: 1; gap: 8px; display: flex; flex-direction: column; flex-wrap: nowrap;">
            <div class="button-bar tabs">
                if *active_view == UnitState::Dec {
                    <button class={classes!("tab", "pressed")} label="dec" onclick={change_view.clone()}>{"Dec"}</button>
                } else {
                    <button class="tab" label="dec" onclick={change_view.clone()}>{"Dec"}</button>
                }

                if *active_view == UnitState::Bin {
                    <button class={classes!("tab", "pressed")} label="bin" onclick={change_view.clone()}>{"Bin"}</button>
                } else {
                    <button class="tab" label="bin" onclick={change_view.clone()}>{"Bin"}</button>
                }

                if *active_view == UnitState::Hex {
                    <button class={classes!("tab", "pressed")} label="hex" onclick={change_view.clone()}>{"Hex"}</button>
                } else {
                    <button class="tab" label="hex" onclick={change_view.clone()}>{"Hex"}</button>
                }
                if !*switch_flag{
                    if *active_view == UnitState::Float {
                        <button class={classes!("tab", "pressed")} label="float" onclick={change_view.clone()}>{"Float"}</button>
                    } else {
                        <button class="tab" label="float" onclick={change_view.clone()}>{"Float"}</button>
                    }
                    if *active_view == UnitState::Double {
                        <button class={classes!("tab", "pressed")} label="double" onclick={change_view.clone()}>{"Double"}</button>
                    } else {
                        <button class="tab" label="double" onclick={change_view.clone()}>{"Double"}</button>
                    }
                }
            </div>
            <div class="button-bar tabs">
            if *switch_flag {
                <button class={classes!("tab", "pressed")} style="width: 50%;" onclick={on_switch_clicked_gp.clone()}>{"GP"}</button>
            } else {
                <button class="tab" style="width: 50%;" onclick={on_switch_clicked_gp.clone()}>{"GP"}</button>
            }
            if !(*switch_flag){
                <button class={classes!("tab", "pressed")} style="width: 50%;" onclick={on_switch_clicked_fp.clone()}>{"FP"}</button>
            } else {
                <button class="tab" style="width: 50%;" onclick={on_switch_clicked_fp.clone()}>{"FP"}</button>
            }
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
                                {generate_gpr_rows(props.gp)}
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
