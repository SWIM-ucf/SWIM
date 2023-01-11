use super::get_options;

use monaco::{yew::{CodeEditor, CodeEditorLink}, api::TextModel};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Editor)]
pub fn editor() -> Html {

    let link: UseStateHandle<CodeEditorLink> = use_state(CodeEditorLink::default);
    let default_code = String::from("");
    let language = String::from("mips");
    let text_model = use_state_eq(|| TextModel::create(&default_code, Some(&language), None).unwrap());
    

    html! {
        <CodeEditor
            classes={css!(
                r#"
                height: 80vh;
                width: 80vw;
                "#
            )}
            options={get_options()}
            link={(*link).clone()}
            model={(*text_model).clone()}
            />
    }
}
