use yew::{function_component, html, Html, Properties};
use crate::emulation_core::mips::memory::{Memory, MemoryIter};
use std::rc::Rc;
use std::cell::RefCell;

use monaco::{
    api::TextModel,
    sys::
        editor::{
            IEditorMinimapOptions, IEditorScrollbarOptions, IStandaloneEditorConstructionOptions, ISuggestOptions
        },
    yew::{CodeEditor, CodeEditorLink},
};
#[derive(PartialEq, Properties)]
pub struct HexEditorProps {
    pub memory_text_model: Rc<RefCell<TextModel>>
}

#[function_component(HexEditor)]
pub fn hex_editor(props: &HexEditorProps) -> Html {
    let editor_link = CodeEditorLink::new();
    html! {
        <CodeEditor
            classes={"editor"}
            link={editor_link}
            options={get_options()}
            model={props.memory_text_model.borrow().clone()}
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
