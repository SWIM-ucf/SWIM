use super::get_options;

use monaco::{api::TextModel, yew::CodeEditor};
use stylist::yew::styled_component;
use yew::prelude::*;

// Stores the state of the text model within the component
// properties so it can be updated and accessed from other components.
#[derive(Eq, PartialEq, Properties)]
pub struct EditorProps {
    pub text_model: TextModel,
}

#[styled_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    // When instantiating the editor, the text model is provided by this
    // component's properties. It is cloned so that ownership is maintained
    // with the properties.

    html! {
        <CodeEditor
            classes={css!(
                r#"
                height: 80vh;
                width: 80vw;
                "#
            )}
            options={get_options()}
            model={props.text_model.clone()}
            />
    }
}
