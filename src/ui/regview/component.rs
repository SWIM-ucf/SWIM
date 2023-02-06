use yew::prelude::*;

use crate::emulation_core::mips::registers::GpRegisters;
use crate::emulation_core::mips::registers::FpRegisters;

// datapath.coprocessor.fpr
#[derive(Properties, PartialEq)]
pub struct Regviewprops {
    pub gp: GpRegisters,
    pub fp: FpRegisters,
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

pub fn gen_reg_fp_html(fp: FpRegisters) -> Html {
    fp.into_iter()
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
    let on_switch_clicked = {
        let switch_view = switch_view.clone();
        use_callback(
            move|_, _| {
                assert_eq!(switch_view, 1);
            }, 
            (),
        )
    };
    html! {
        <>
        <button onclick={on_switch_clicked}>{"Switch view"}</button>
            <div>
                <table style="width: 19.2%; height: 97vh; border: 1px solid black; background-color: white; float: right;" content="width=device-width; initial-scale=1.0">
                    <tr style="border: 1px solid black;">
                        <th style="border: 1px solid black;">{"Register Name"}</th>
                        <th style="border: 1px solid black;">{"Data"}</th>
                    </tr>
                    {gen_reg_html(props.gp)}
                </table>
            </div>
        </>
    }
}
