use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Consoleprops {
    pub parsermsg: String,
}

#[function_component(Console)]
pub fn console(props: &Consoleprops) -> Html {
    html! {
        <>
            // <div style="width: 80.4%; height: 17vh; border: 1px solid black;"></div>
            <div style="height: 17vh; border: 2px solid black; background-color: #b9cceb; color: #000000;">
                <p>{ props.parsermsg.clone() }</p>
                //<p>{ ">" }</p>
            </div>
        </>
    }
}
