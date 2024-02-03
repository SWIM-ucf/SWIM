use std::{cell::RefCell, rc::Rc};

use monaco::{api::TextModel, sys::editor::{
    IEditorMinimapOptions, IEditorScrollbarOptions, IStandaloneEditorConstructionOptions, ISuggestOptions, ScrollType
}, yew::{CodeEditor, CodeEditorLink}};
use yew::{Callback, Properties};
use yew::prelude::*;
use wasm_bindgen::JsCast;

use crate::{parser::parser_structs_and_enums::ProgramInfo, ui::assembled_view::component::{DataSegment, TextSegment}};

use log::debug;

#[derive(PartialEq, Properties)]
pub struct SwimEditorProps {
    pub text_model: Rc<RefCell<TextModel>>,
    pub lines_content: Rc<RefCell<Vec<String>>>,
    pub program_info: ProgramInfo,
    pub binary: Vec<u32>,
    pub pc: u64,
    pub memory_curr_line: UseStateHandle<f64>,
    pub editor_curr_line: UseStateHandle<f64>
}

#[derive(Default, PartialEq)]
pub enum EditorTabState {
    #[default]
    Editor,
    TextSegment,
    DataSegment
}

fn get_options() -> IStandaloneEditorConstructionOptions {
    let options = IStandaloneEditorConstructionOptions::default();
    options.set_theme("vs-dark".into());
    options.set_language("mips".into());
    options.set_scroll_beyond_last_line(false.into());
    options.set_automatic_layout(true.into());

    let minimap = IEditorMinimapOptions::default();
    minimap.set_enabled(false.into());
    options.set_minimap(Some(&minimap));

    let scrollbar = IEditorScrollbarOptions::default();
    scrollbar.set_always_consume_mouse_wheel(false.into());
    options.set_scrollbar(Some(&scrollbar));

    let suggest = ISuggestOptions::default();
    suggest.set_show_keywords(false.into());
    suggest.set_show_variables(false.into());
    suggest.set_show_icons(false.into());
    suggest.set_show_words(false.into());
    suggest.set_filter_graceful(false.into());
    options.set_suggest(Some(&suggest));

    options
}

#[function_component]
pub fn SwimEditor(props: &SwimEditorProps) -> Html {
    let link = CodeEditorLink::new();

    let on_editor_created = {
        let curr_line = props.editor_curr_line.clone();
        let lines_content = Rc::clone(&props.lines_content);

        use_callback(
            move |editor_link: CodeEditorLink, curr_line| {
                match editor_link.with_editor(|editor| {
                    let raw_editor = editor.as_ref();
                    let model = raw_editor.get_model().unwrap();
                    // store each line from the original code editor's contents for assembled view
                    let js_lines = model.get_lines_content();
                    let mut string_lines = lines_content.borrow_mut();
                    for js_string in js_lines.into_iter() {
                        let string_value = match js_string.as_string() {
                            Some(string) => string,
                            None => String::from("")
                        };
                        string_lines.push(string_value);

                    };
                    raw_editor.reveal_line_in_center(**curr_line, Some(ScrollType::Smooth));
                }) {
                    Some(()) => debug!("Editor linked!"),
                    None => debug!("No editor :<")
                };
            },
            curr_line,
        )
    };

    let active_tab = use_state_eq(EditorTabState::default);
    let change_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
            let tab_name = target
                .get_attribute("label")
                .unwrap_or(String::from("editor"));

            let new_tab: EditorTabState = match tab_name.as_str() {
                "editor" => EditorTabState::Editor,
                "text" => EditorTabState::TextSegment,
                "data" => EditorTabState::DataSegment,
                _ => EditorTabState::default(),
            };

            active_tab.set(new_tab);
        })
    };
    html! {
        <>
            // Editor buttons
            <div class="bar tabs">
                if *active_tab == EditorTabState::Editor {
                    <button class={classes!("tab", "pressed")} label="editor" onclick={change_tab.clone()}>{"Editor"}</button>
                } else {
                    <button class="tab" label="editor" onclick={change_tab.clone()}>{"Editor"}</button>
                }

                if *active_tab == EditorTabState::TextSegment {
                    <button class={classes!("tab", "pressed")} label="text" onclick={change_tab.clone()}>{"Text Segment"}</button>
                } else {
                    <button class="tab" label="text" onclick={change_tab.clone()}>{"Text Segment"}</button>
                }

                if *active_tab == EditorTabState::DataSegment {
                    <button class={classes!("tab", "pressed")} label="data" onclick={change_tab.clone()}>{"Data Segment"}</button>
                } else {
                    <button class="tab" label="data" onclick={change_tab.clone()}>{"Data Segment"}</button>
                }
            </div>
            if *active_tab == EditorTabState::Editor {
                <CodeEditor classes={"editor"} link={link.clone()} options={get_options()} model={props.text_model.borrow().clone()} on_editor_created={on_editor_created.clone()}/>
            } else if *active_tab == EditorTabState::TextSegment {
                <TextSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} pc={props.pc.clone()} active_tab={active_tab.clone()} memory_curr_line={props.memory_curr_line.clone()} editor_curr_line={props.editor_curr_line.clone()}/>
            } else if *active_tab == EditorTabState::DataSegment {
                <DataSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} binary={props.binary.clone()}/>
            }
        </>
    }
}