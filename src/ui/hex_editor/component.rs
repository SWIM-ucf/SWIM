use log::debug;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};
use yew::{function_component, html, use_callback, Html, Properties, UseStateHandle};
use yew::prelude::*;

use monaco::{
    api::TextModel,
    sys::{
        editor::{
            IEditorMinimapOptions, IEditorScrollbarOptions, IModelDecorationOptions,
            IModelDeltaDecoration, IStandaloneEditorConstructionOptions, ISuggestOptions,
            ScrollType,
        },
        Range,
    },
    yew::{CodeEditor, CodeEditorLink},
};

#[derive(PartialEq, Properties)]
pub struct HexCoord {
    pub line_number: f64,
    pub start_column: f64,
    pub end_column: f64,
}

#[derive(PartialEq, Properties)]
pub struct HexEditorProps {
    pub memory_text_model: UseStateHandle<TextModel>,
    // The instruction to highlight
    pub instruction_num: UseStateHandle<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdatedLine {
    pub text: String,
    pub line_number: usize
}
impl UpdatedLine {
    pub fn new(text: String, line_number: usize) -> Self {
        UpdatedLine {
            text,
            line_number
        }
    }
}

#[function_component(HexEditor)]
pub fn hex_editor(props: &HexEditorProps) -> Html {
    let editor_link = CodeEditorLink::new();
    // Program counter - will probably change
    let instruction_num = *props.instruction_num;
    let text_model = &props.memory_text_model;
    // Store highlight decoration IDs
    let decorations = use_mut_ref(js_sys::Array::new);

    // create a JavaScript closure for hex highlighting
    let text_model_ref = text_model.clone();
    let cb = Closure::new(Box::new(
        move |event: monaco::sys::editor::ICursorSelectionChangedEvent| {
            // Get a mutable reference to decorations
            let decorations = Rc::clone(&decorations);
            let mut decorations = decorations.borrow_mut();

            // Create the ASCII highlight range
            let selection = event.selection();
            let start_line_number = selection.selection_start_line_number();
            let start_column = selection.start_column();
            let end_column = selection.end_column();
            let (start_column, end_column) = calculate_ascii_columns(start_column as usize, end_column as usize);
            let range = Range::new(
                start_line_number,
                start_column,
                start_line_number,
                end_column,
            );

            // Style the highlighting
            let highlight_decoration: IModelDeltaDecoration = js_sys::Object::new().unchecked_into();
            let highlight_options: IModelDecorationOptions = js_sys::Object::new().unchecked_into();
            highlight_options.set_inline_class_name("highlightHex".into());
            highlight_options.set_is_whole_line(false.into());
            highlight_decoration.set_options(&highlight_options);
            let range_js = range
                .dyn_into::<JsValue>()
                .expect("Range is not found.");
            highlight_decoration.set_range(&monaco::sys::IRange::from(range_js));
            let decoration_js = highlight_decoration
                .dyn_into::<JsValue>()
                .expect("Highlight is not found.");

            // Create JS Arrays
            let not_highlighted = js_sys::Array::new();
            let executed_line = js_sys::Array::new();
            executed_line.push(&decoration_js);

            // Get the monaco text model
            let text_model = text_model_ref.clone();
            let text_model = text_model.as_ref();
            // Clear previous highlights
            let existing_decorations = text_model.get_all_decorations(None, None);
            text_model.delta_decorations(&decorations, &not_highlighted, None);
            // Set new decorations and save their IDs
            *decorations = text_model.delta_decorations(&existing_decorations, &executed_line, None);
        },
    ) as Box<dyn FnMut(_)>);

    // Returns a struct containing monaco-like coordinates (start and end line numbers and columns)
    // given the program counter (index of a WORD)
    fn get_hex_coords(instruction_num: u64) -> HexCoord {
        let line_number = instruction_num / 16 + 1;
        let offset = 10;
        let start_column = offset + ((instruction_num % 16) * 2 + ((instruction_num % 16) / 4));
        let end_column = start_column + 8;

        let coords = HexCoord {
            line_number: line_number as f64,
            start_column: start_column as f64,
            end_column: end_column as f64,
        };

        coords
    }

    // Calculates which columns in the ASCII portion belong to the given hex portion
    fn calculate_ascii_columns(hex_start_column: usize, hex_end_column: usize) -> (f64, f64) {
        if hex_start_column > 8 && hex_start_column < 46 && hex_end_column > 8 && hex_end_column < 46 && hex_end_column > hex_start_column {
            let ascii_length = (hex_end_column - hex_start_column) / 2;
            let start_column = 46 + ((hex_start_column - 8) / 2) - 1;
            let end_column = start_column + ascii_length;
            (start_column as f64, end_column as f64)
        } else {
            (0.0, 0.0)
        }
    }

    let on_editor_created = {
        use_callback(
            move |editor_link: CodeEditorLink, instruction_num| {
                match editor_link.with_editor(|editor| {
                    let raw_editor = editor.as_ref();
                    let cb_func = &cb.as_ref().unchecked_ref();

                    let coords = get_hex_coords(*instruction_num);
                    raw_editor.on_did_change_cursor_selection(cb_func);
                    raw_editor.reveal_line_in_center(coords.line_number, Some(ScrollType::Smooth));

                    // Highlight line using delta decorations
                    let not_highlighted = js_sys::Array::new();
                    let executed_line = js_sys::Array::new();
                    let decoration: IModelDeltaDecoration = js_sys::Object::new().unchecked_into();
                    let options: IModelDecorationOptions = js_sys::Object::new().unchecked_into();
                    if coords.line_number != 0.0 {
                        // Show highlight if current line is not 0
                        options.set_inline_class_name("executedLine".into());
                        options.set_is_whole_line(false.into());
                    }
                    decoration.set_options(&options);
                    let curr_range = Range::new(
                        coords.line_number,
                        coords.start_column,
                        coords.line_number,
                        coords.end_column,
                    );
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
                    Some(()) => debug!("Hex Editor linked!"),
                    None => debug!("No editor :<"),
                };
            },
            instruction_num,
        )
    };
    html! {
        <CodeEditor
            classes={"editor"}
            link={editor_link}
            options={get_options()}
            model={text_model.deref().clone()}
            on_editor_created={on_editor_created}
        />
    }
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

pub fn parse_hexdump(input: &str) -> Result<Vec<u32>, String> {
    let mut words = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        for &part in &parts[2..6] {
            let data = u32::from_str_radix(part, 16).map_err(|e| e.to_string())?;
            words.push(data);
        }
    }
    Ok(words)
}