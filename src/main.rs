pub mod emulation_core;
#[cfg(test)]
pub mod tests;
pub mod ui;

use stylist::yew::*;
use ui::editor::component::Editor;
use ui::regview::component::Regview;
use ui::console::component::Console;
use yew::prelude::*;

#[styled_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Welcome to SWIM"}</h1>
            <Regview />
            <Editor /> 
            <Console />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
