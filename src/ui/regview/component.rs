use yew::prelude::*;

use crate::emulation_core::mips::registers::GpRegisters;

#[derive(Properties, PartialEq)]
pub struct Regviewprops {
    pub gp: GpRegisters,
}

//Convert register to html through iterator
pub fn genRegHtml(gp: GpRegisters) -> Html {
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
#[function_component(Regview)]
pub fn regview(props: &Regviewprops) -> Html {
    html! {
        <>
            <div>
                <table style="width: 19.2%; height: 97vh; border: 1px solid black; background-color: white; float: right;" content="width=device-width; initial-scale=1.0">
                    <tr style="border: 1px solid black;">
                        <th style="border: 1px solid black;">{"Register Name"}</th>
                        <th style="border: 1px solid black;">{"Data"}</th>
                    </tr>
                    {genRegHtml(props.gp)}
                </table>
            </div>
        </>
    }
}
