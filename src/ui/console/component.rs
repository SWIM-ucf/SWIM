use yew::prelude::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::{print_vec_of_instructions, Error, self, ProgramInfo};

#[derive(Properties, PartialEq)]
pub struct Consoleprops {
    pub parsermsg: String,
    pub memorymsg: String
}

/*pub fn mem_reg_html(memory: Vec<u8>) -> Html {

}*/

#[function_component(Console)]
pub fn console(props: &Consoleprops) -> Html {
    let parser_text_output = use_state_eq(String::new);
    let switch_flag = use_state_eq(|| 0);
    // let on_switch_clicked_0 = {
    //     let switch_flag = switch_flag.clone();
    //     use_callback(
    //         move |_, switch_flag| {
    //             if **switch_flag != 0 {
    //                 switch_flag.set(0);
    //             }
    //         },
    //         switch_flag,
    //     )
    // };
    // let on_switch_clicked_1 = {
    //     let switch_flag = switch_flag.clone();
    //     use_callback(
    //         move |_, switch_flag| {
    //             if **switch_flag != 1 {
    //                 switch_flag.set(1);
    //             }
    //         },
    //         switch_flag,
    //     )
    // };
    // let on_switch_clicked_2 = {
    //     let switch_flag = switch_flag.clone();
    //     use_callback(
    //         move |_, switch_flag| {
    //             if **switch_flag != 2 {
    //                 switch_flag.set(2);
    //             }
    //         },
    //         switch_flag,
    //     )
    // };
    //let instructions = ProgramInfo::new();
    /*let on_error_clicked = {
        let parser_text_output = parser_text_output.clone();
        use_callback(
            move |_, _| {
                parser_text_output.set(print_vec_of_instructions(instructions).to_string());
            },
            (),
        )
    }; */
    html! {
        <>
            <div style="flex-basis: 50%; border: 2px solid black; background-color: #b9cceb; color: #000000; overflow-y: auto;">
                { props.parsermsg.clone() }
            </div>
        </>
    }
}
