use std::{cell::RefCell, rc::Rc};

use monaco::{api::TextModel, sys::{editor::{
    IEditorMinimapOptions, IEditorScrollbarOptions, IModelDecorationOptions, IModelDeltaDecoration, IStandaloneEditorConstructionOptions, ISuggestOptions, ScrollType
}, Range}, yew::{CodeEditor, CodeEditorLink}};
use yew::{Callback, Properties};
use yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use crate::{parser::parser_structs_and_enums::ProgramInfo, ui::assembled_view::component::{DataSegment, TextSegment}};

use log::debug;

#[derive(PartialEq, Properties)]
pub struct SwimEditorProps {
    pub text_model: UseStateHandle<TextModel>,
    pub lines_content: Rc<RefCell<Vec<String>>>,
    pub program_info: ProgramInfo,
    pub binary: Vec<u32>,
    pub pc: u64,
    pub memory_curr_line: UseStateHandle<f64>,
    pub editor_curr_line: UseStateHandle<f64>,
    pub active_tab: UseStateHandle<EditorTabState>
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
    let text_model = &*props.text_model;

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
                    // Scroll to current line
                    raw_editor.reveal_line_in_center(**curr_line, Some(ScrollType::Smooth));
                    // Highlight current line using delta decorations
                    let not_highlighted = js_sys::Array::new();
                    let executed_line = js_sys::Array::new();
                    let decoration: IModelDeltaDecoration = js_sys::Object::new().unchecked_into();
                    let options: IModelDecorationOptions = js_sys::Object::new().unchecked_into();
                    if **curr_line != 0.0 {
                        // Show highlight if current line is not 0
                        options.set_inline_class_name("myInlineDecoration".into());
                        options.set_is_whole_line(true.into());
                    }
                    decoration.set_options(&options);
                    let curr_range = Range::new(**curr_line, 0.0, **curr_line, 0.0);
                    let range_js = curr_range
                        .dyn_into::<JsValue>()
                        .expect("Range is not found.");
                    decoration.set_range(&monaco::sys::IRange::from(range_js));
                    let decoration_js = decoration
                        .dyn_into::<JsValue>()
                        .expect("Highlight is not found.");
                    executed_line.push(&decoration_js);
                    raw_editor.delta_decorations(&not_highlighted, &executed_line);
                }) {
                    Some(_) => (),
                    None => ()
                };
            },
            curr_line,
        )
    };

    let active_tab = &props.active_tab;
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
                if **active_tab == EditorTabState::Editor {
                    <button class={classes!("tab", "pressed")} label="editor" onclick={change_tab.clone()}>{"Editor"}</button>
                } else {
                    <button class="tab" label="editor" onclick={change_tab.clone()}>{"Editor"}</button>
                }

                if **active_tab == EditorTabState::TextSegment {
                    <button class={classes!("tab", "pressed")} label="text" onclick={change_tab.clone()}>{"Text Segment"}</button>
                } else {
                    <button class="tab" label="text" onclick={change_tab.clone()}>{"Text Segment"}</button>
                }

                if **active_tab == EditorTabState::DataSegment {
                    <button class={classes!("tab", "pressed")} label="data" onclick={change_tab.clone()}>{"Data Segment"}</button>
                } else {
                    <button class="tab" label="data" onclick={change_tab.clone()}>{"Data Segment"}</button>
                }
            </div>
            if **active_tab == EditorTabState::Editor {
                <CodeEditor classes={"editor"} link={link.clone()} options={get_options()} model={text_model.clone()} on_editor_created={on_editor_created.clone()}/>
            } else if **active_tab == EditorTabState::TextSegment {
                <TextSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} pc={props.pc.clone()} active_tab={active_tab.clone()} memory_curr_line={props.memory_curr_line.clone()} editor_curr_line={props.editor_curr_line.clone()}/>
            } else if **active_tab == EditorTabState::DataSegment {
                <DataSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} binary={props.binary.clone()}/>
            }
        </>
    }
}