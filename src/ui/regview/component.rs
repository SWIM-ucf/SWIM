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
pub fn gen_reg_html(gp: GpRegisters) -> Html {
    gp.into_iter()
        .map(|(register, data)| {
            html! {
                <tr style="border: 1px solid black;">
                    <td style="border: 1px solid black;">
                        {register}
                    </td>
                    <td style="border: 1px solid black;">
                        {data.to_string()}
                    </td>
                </tr>
            }
        })
        .collect::<Html>()
}

pub fn fp_reg(fp: [u64; 32]) -> Html {
    fp.iter()
        .enumerate()
        .map(|(register, data)| {
            html! {
                <tr style="border: 1px solid black;">
                    <td style="border: 1px solid black;">
                        {register}
                    </td>
                    <td style="border: 1px solid black;">
                        {data.to_string()}
                    </td>
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
        <>
            <div style="width: 28%">
                <div class="tab">
                    <button class="tabs" style="width: 50%;"
                    onclick={on_switch_clicked_true.clone()}>{"GP"}</button>
                    <button class="tabs" style="width: 50%;"
                    onclick={on_switch_clicked_false.clone()}>{"FP"}</button>
                </div>
                <table style="width: 100%; height: 100%; border: 1px solid black; background-color: white">
                    <tr style="border: 1px solid black;">
                        <th style="border: 1px solid black;">{"Register Name"}</th>
                        <th style="border: 1px solid black;">{"Data"}</th>
                    </tr>
                    if *switch_flag{
                        {gen_reg_html(props.gp)}
                    } else {
                        {fp_reg(props.fp)}
                    }
                </table>
            </div>
        </>
    }
}
