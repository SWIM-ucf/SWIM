use js_sys::Object;
use wasm_bindgen::{closure::Closure, JsValue};
use yew::{function_component, html, Html, Properties, use_callback, Callback, use_mut_ref, use_effect_with_deps};
use crate::emulation_core::mips::memory::{Memory, MemoryIter};
use std::rc::Rc;
use std::cell::RefCell;
use log::debug;
use wasm_bindgen::JsCast;

use monaco::{
    api::TextModel,
    sys::
        editor::{
            IEditorMinimapOptions, IEditorScrollbarOptions, IStandaloneEditorConstructionOptions, ISuggestOptions, ScrollType
        },
    yew::{CodeEditor, CodeEditorLink},
};
#[derive(PartialEq, Properties)]
pub struct HexEditorProps {
    pub memory_text_model: Rc<RefCell<TextModel>>,
    pub curr_line: Rc<RefCell<f64>>
}

#[function_component(HexEditor)]
pub fn hex_editor(props: &HexEditorProps) -> Html {
    let editor_link = CodeEditorLink::new();
    let text_model = Rc::clone(&props.memory_text_model);
    let curr_line = Rc::clone(&props.curr_line);
    let not_highlighted = js_sys::Array::new();
    let mut mutated = false;

    // create a JavaScript closure
    let cb = Closure::wrap(Box::new(move |event: monaco::sys::editor::ICursorSelectionChangedEvent| {
        let highlight_decor = monaco::sys::editor::IModelDecorationOptions::default();
        highlight_decor.set_class_name("hexHighlight".into());

        debug!("Selection: {:?}", event.selection());
        let selection = event.selection();
        let start_line_number = selection.selection_start_line_number();
        let end_line_number = selection.position_line_number();
        let start_column = selection.start_column();
        let end_column = selection.end_column();

        let curr_range = monaco::sys::Range::new(start_line_number, start_column, end_line_number, end_column);

        // element to be stored in the stack to highlight the line
        let highlight_line: monaco::sys::editor::IModelDeltaDecoration =
            Object::new().unchecked_into();
        highlight_line.set_options(&highlight_decor);
        let range_js = curr_range
            .dyn_into::<JsValue>()
            .expect("Hex range is not found.");
        highlight_line.set_range(&monaco::sys::IRange::from(range_js));
        let highlight_js = highlight_line
            .dyn_into::<JsValue>()
            .expect("Hex highlight is not found.");

        if start_column > 8.0 && end_column < 46.0 {
            // select ASCII
        }
        else if start_column > 45.0 && end_column < 63.0 {
            // select hex
        }


    }) as Box<dyn FnMut(_)>);

    let on_editor_created = {
        let text_model = Rc::clone(&props.memory_text_model);
        let curr_line = Rc::clone(&curr_line);

        use_callback(
            move |editor_link: CodeEditorLink, text_model| {
                let curr_line = curr_line.borrow_mut();
                match editor_link.with_editor(|editor| {
                    let raw_editor = editor.as_ref();

                    debug!("Helo!");
                    let cb_func = &cb.as_ref().unchecked_ref();

                    raw_editor.on_did_change_cursor_selection(cb_func);
                    // raw_editor.reveal_line_in_center(*curr_line, Some(ScrollType::Smooth))

                }) {
                    Some(()) => debug!("Hex Editor linked!"),
                    None => debug!("No editor :<")
                };
            },
            text_model,
        )
    };
    html! {
        <CodeEditor
            classes={"editor"}
            link={editor_link}
            options={get_options()}
            model={props.memory_text_model.borrow().clone()}
            on_editor_created={on_editor_created}
        />
    }
}

pub fn generate_formatted_hex(memory: &Memory) -> String {
    let iterator = MemoryIter::new(&memory);

    let mut string: String = "".to_string();

    for (address, words) in iterator {
        string.push_str(&format!("0x{address:04x}:\t\t"));
        let mut char_version: String = "".to_string();

        for word in words {
            string.push_str(&format!("{:08x}\t", word));
            char_version.push_str(&convert_word_to_chars(word));
        }

        string.push_str(&format!("{char_version}\n"));
    }

    string
}

fn convert_word_to_chars(word: u32) -> String {
    let mut chars = "".to_string();
    for shift in (0..4).rev() {
        let byte = (word >> (shift * 8)) as u8;
        if byte > 32 && byte < 127 {
            chars.push(byte as char);
        } else {
            chars.push('.');
        }
    }
    chars
}


fn get_options() -> IStandaloneEditorConstructionOptions {
    let options = IStandaloneEditorConstructionOptions::default();
    options.set_theme("vs-dark".into());
    options.set_language("ini".into());
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
