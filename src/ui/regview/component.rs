use yew::prelude::*;

#[function_component(Regview)]
pub fn regview() -> Html {
    html! {
        <>
            <div>
                <table style="width: 365px; height: 734px; border: 1px solid black; background-color: white; float: right;">
                    <tr style="border: 1px solid black;">
                        <th style="border: 1px solid black;">{"GP"}</th>
                        <th style="border: 1px solid black;">{"FP"}</th>
                    </tr>
                    <tr style="border: 1px solid black;">
                        <td style="border: 1px solid black;"></td>
                        <td style="border: 1px solid black;"></td>
                    </tr>
                    <tr style="border: 1px solid black;">
                        <td style="border: 1px solid black;"></td>
                        <td style="border: 1px solid black;"></td>
                    </tr>
                </table>
            </div>
        </>
    }
}