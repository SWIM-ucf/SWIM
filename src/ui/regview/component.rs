use gloo::console::log;
use yew::prelude::*;

use crate::emulation_core::mips::registers::GpRegisters;

// datapath.coprocessor.fpr
#[derive(Properties, PartialEq)]
pub struct Regviewprops {
    pub gp: GpRegisters,
    pub fp: [u64; 32],
}

#[derive(Properties, PartialEq)]
pub struct Viewswitch {
    pub switch_view: bool,
}

//Convert register to html through iterator
pub fn generate_gpr_rows(gp: GpRegisters) -> Html {
    gp.into_iter()
        .map(|(register, data)| {
            html! {
                <tr>
                    <td>{register}</td>
                    <td>{data.to_string()}</td>
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
                    <td>{data.to_string()}</td>
                </tr>
            }
        })
        .collect::<Html>()
}

#[function_component(Regview)]
pub fn regview(props: &Regviewprops) -> Html {
    let switch_flag = use_state_eq(|| true);
    let on_switch_clicked_false = {
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
    let on_switch_clicked_true = {
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
    log!("This is ", *switch_flag);
    html! {
        <div style="flex-grow: 1; gap: 8px; display: flex; flex-direction: column; flex-wrap: nowrap;">
            <div class="tabs">
                <button class="tab" style="width: 50%;" onclick={on_switch_clicked_true.clone()}>{"GP"}</button>
                <button class="tab" style="width: 50%;" onclick={on_switch_clicked_false.clone()}>{"FP"}</button>
            </div>
            <div class="table-wrapper">
                <table>
                    <thead>
                        <tr>
                            <th>{"Register Name"}</th>
                            <th>{"Data"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        if *switch_flag{
                            {generate_gpr_rows(props.gp)}
                        } else {
                            {generate_fpr_rows(props.fp)}
                        }
                    </tbody>
                </table>
            </div>
        </div>
    }
}
