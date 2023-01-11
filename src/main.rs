pub mod emulation_core;
pub mod parser;
#[cfg(test)]
pub mod tests;
pub mod ui;

use stylist::yew::*;
use ui::editor::component::Editor;
use yew::prelude::*;

#[styled_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Welcome to SWIM"}</h1>
            <Editor />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
