pub mod emulation_core;
#[cfg(test)]
pub mod tests;
pub mod ui;

use stylist::yew::*;
use ui::editor::component::Editor;
use ui::components::buttons::execute_button::ExecuteButton;
use yew::prelude::*;

#[styled_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Welcome to SWIM"}</h1>
            <ExecuteButton />
            <Editor />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
