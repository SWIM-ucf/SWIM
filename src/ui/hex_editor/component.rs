use yew::{function_component, html, Html, Properties, TargetCast, Component, Context};
use crate::emulation_core::mips::memory::{Memory, MemoryIter};
use yew::prelude::*;
use web_sys::Element;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlInputElement, InputEvent,
};
use std::rc::Rc;
use std::cell::RefCell;
use log::debug;
use js_sys::{Array, Object, Function};
use yew_hooks::prelude::*;

use std::ops::{Deref, DerefMut};

use monaco::{
    api::TextModel,
    sys::{
        editor::{
            IEditorMinimapOptions, IEditorScrollbarOptions, IMarkerData, IModelDecorationOptions, ICursorPositionChangedEvent, IStandaloneCodeEditor,
            IModelDeltaDecoration, IStandaloneEditorConstructionOptions, ISuggestOptions, ICodeEditor, create
        },
        IMarkdownString, MarkerSeverity,
        languages::{ILanguageExtensionPoint, LanguageConfiguration}
    },
    yew::{CodeEditor, CodeEditorLink},
};

pub struct CursorPosition {
    line_number: u32,
    column_number: u32
}
#[derive(PartialEq, Properties)]
pub struct HexEditorProps {
    // #[prop_or_default]
    // pub memory: Memory
    pub memory_text_model: Rc<RefCell<TextModel>>
}

#[function_component(HexEditor)]
pub fn hex_editor(props: &HexEditorProps) -> Html {
    // let memory_string = generate_formatted_hex(&props.memory);
    
    // let memory_text_model = use_mut_ref(|| TextModel::create(&memory_string, Some("ini"), None).unwrap());

    
    // let dirty_line = js_sys::Array::new();

    // let cursor_position = CursorPosition {
    //     line_number: 0,
    //     column_number: 0
    // };

    let editor_link = CodeEditorLink::new();

    // Define the event listener
    // pub fn handle_cursor_position_change (e: ICursorPositionChangedEvent) {
    //     debug!("{:?}", e.position());
    // };

    // let editor_link_clone = editor_link.clone();

    // // Use the use_effect hook to run code after the component is mounted
    // use_effect(move || {
    //     // Access the CodeEditor instance after it's created
    //     if let Some(editor) = editor_link_clone.with_editor(|editor| {
    //         // Attach the event listener after the editor is created
    //         editor.on_did_change_cursor_position(handle_cursor_position_change.clone())
    //     }) {
    //         // Perform cleanup if needed
            
    //     }
    // });

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


// editor.onDidChangeModelContent(e => {
//     if (/* your condition here */) {
//         // your logic here
//         overridenPosition = { lineNumber: 4, column: 2 }; // put your value here
//     }
// });

// editor.onDidChangeCursorPosition(e => {
//     if (overridenPosition != null) {
//         editor.setPosition(overridenPosition);
//         overridenPosition = null;
//     }
// });

// pub struct HexEditor {
//     node_ref: NodeRef,
// }

// impl Component for HexEditor {
//     type Message = ();
//     type Properties = ();

//     // Properties from the parent component are store in Context<Self>
//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             node_ref: NodeRef::default(),
//         }
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <div ref={self.node_ref.clone()}></div>
//         }
//     }

//     fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
//         let has_attributes = self.node_ref
//             .cast::<Element>()
//             .unwrap()
//             .has_attributes();
//     }
// }
// pub enum Msg {
//     UpdateValue(String),
//     UpdateAddress(String),
// }

