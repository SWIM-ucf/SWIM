use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};
use yew::prelude::*;
use yew::{function_component, html, use_callback, Html, Properties, UseStateHandle};

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

use crate::emulation_core::mips::memory::{Memory, CAPACITY_BYTES};

#[derive(PartialEq, Properties)]
pub struct HexCoord {
    pub line_number: f64,
    pub start_column: f64,
    pub end_column: f64,
}

#[derive(PartialEq, Properties)]
pub struct HexEditorProps {
    pub memory_text_model: UseStateHandle<TextModel>,
    pub memory: Memory,
    pub pc: u64,
    // The instruction to highlight
    pub memory_curr_instr: UseStateHandle<u64>,
    pub initialized: bool,
    pub executing: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdatedLine {
    pub text: String,
    pub line_number: usize,
}
impl UpdatedLine {
    pub fn new(text: String, line_number: usize) -> Self {
        UpdatedLine { text, line_number }
    }
}

#[function_component(HexEditor)]
pub fn hex_editor(props: &HexEditorProps) -> Html {
    let editor_link = CodeEditorLink::new();
    let memory_text_model = &props.memory_text_model;

    // Store highlight decoration IDs
    let decorations = use_mut_ref(js_sys::Array::new);

    // create a JavaScript closure for hex highlighting
    let text_model_ref = memory_text_model.clone();
    let cb = Closure::new(Box::new(
        move |event: monaco::sys::editor::ICursorSelectionChangedEvent| {
            // Get a mutable reference to decorations
            let decorations = Rc::clone(&decorations);
            let mut decorations = decorations.borrow_mut();

            // Get the monaco text model
            let memory_text_model = text_model_ref.clone();
            let memory_text_model_ref = memory_text_model.as_ref();

            // Clear previous highlights
            // Create JS Array with an empty decoration
            let not_highlighted = js_sys::Array::new();
            memory_text_model_ref.delta_decorations(&decorations, &not_highlighted, None);

            // Create the ASCII highlight range
            let selection = event.selection();
            let start_line_number = selection.selection_start_line_number();
            let end_line_number = selection.end_line_number();
            let start_column = selection.start_column();
            let end_column = selection.end_column();

            // Get current line's contents
            let line_number = event.selection().selection_start_line_number();
            let line_content = memory_text_model_ref.get_line_content(line_number);
            let mut line_strings = line_content.split_whitespace().collect::<Vec<&str>>();
            // remove first element if it is an address
            let address = line_strings.remove(0);
            // save the address length to calculate the actual selection later
            let address_length = address.len();

            // doesn't support multi-line highlighting yet
            if start_column <= address_length as f64
                || end_column <= address_length as f64 + 2.0
                || start_column > 46.0
                || end_column > 46.0
                || end_column <= start_column
                || start_line_number != end_line_number
            {
                return;
            }

            // count whitespaces in line up to selection
            let mut whitespace_count = 0;
            for (i, c) in line_content.chars().enumerate() {
                if i >= start_column as usize {
                    break;
                }
                if c == ' ' || c == '\t' {
                    whitespace_count += 1;
                }
            }

            // count whitespaces in selection
            let selection_slice = &line_content[start_column as usize - 1..end_column as usize - 1];
            let mut whitespace_count_selection = 0;
            for c in selection_slice.chars() {
                if c == ' ' || c == '\t' {
                    whitespace_count_selection += 1;
                }
            }
            // separate selection into valid bytes
            // for example, if the selection is "7bd" in the word "27bdffd8", only "bd" is a valid byte to convert
            let mut actual_start_col = start_column as usize - whitespace_count - address_length;
            let mut actual_end_col = end_column as usize
                - whitespace_count
                - whitespace_count_selection
                - address_length;
            // if the first bit is part of an incomplete byte, remove it
            if actual_start_col % 2 == 0 {
                actual_start_col += 1;
            }
            // if the last bit is part of an incomplete byte, remove it
            if actual_end_col % 2 == 0 {
                actual_end_col -= 1;
            }
            // make sure the resulting selection is valid
            if actual_end_col > actual_start_col {
                // uncomment to see the selection converted to ASCII
                // // convert the selection to ASCII two bits at a time
                // let no_whitespace_line = line_strings.join("");
                // let new_selection = &no_whitespace_line[actual_start_col - 1..actual_end_col - 1];
                // let mut converted_hex = String::new();
                // for (i, _c) in new_selection.chars().enumerate().step_by(2) {
                //     if (i + 1) >= new_selection.len() {
                //         break;
                //     }
                //     let ascii_digits = &new_selection[i..i + 2];
                //     let ascii_digits = u8::from_str_radix(ascii_digits, 16).unwrap();
                //     let ascii_str = ascii_digits as char;
                //     converted_hex.push(ascii_str);
                // }

                // Create the ASCII highlight range
                let ascii_start_column = 46 + (actual_start_col / 2);
                let ascii_end_column = 46 + (actual_end_col / 2);

                let range = Range::new(
                    start_line_number,
                    ascii_start_column as f64,
                    start_line_number,
                    ascii_end_column as f64,
                );

                // Style the highlighting
                let highlight_decoration: IModelDeltaDecoration =
                    js_sys::Object::new().unchecked_into();
                let highlight_options: IModelDecorationOptions =
                    js_sys::Object::new().unchecked_into();
                highlight_options.set_inline_class_name("highlightHex".into());
                highlight_options.set_is_whole_line(false.into());
                highlight_decoration.set_options(&highlight_options);
                let range_js = range.dyn_into::<JsValue>().expect("Range is not found.");
                highlight_decoration.set_range(&monaco::sys::IRange::from(range_js));
                let decoration_js = highlight_decoration
                    .dyn_into::<JsValue>()
                    .expect("Highlight is not found.");

                // Create JS Array with the new decoration
                let executed_line = js_sys::Array::new();
                executed_line.push(&decoration_js);

                // Get the monaco text model
                let memory_text_model = text_model_ref.clone();
                let memory_text_model_ref = memory_text_model.as_ref();

                let existing_decorations = memory_text_model_ref.get_all_decorations(None, None);
                // Set new decorations and save their IDs
                *decorations = memory_text_model_ref.delta_decorations(
                    &existing_decorations,
                    &executed_line,
                    None,
                );
            }
        },
    ) as Box<dyn FnMut(_)>);

    // Returns a struct containing monaco-like coordinates (start and end line numbers and columns)
    // given the program counter (index of a WORD)
    fn get_hex_coords(memory_curr_instr: u64) -> HexCoord {
        let line_number = memory_curr_instr / 16 + 1;
        let offset = 10;
        let start_column = offset + ((memory_curr_instr % 16) * 2 + ((memory_curr_instr % 16) / 4));
        let end_column = start_column + 8;

        HexCoord {
            line_number: line_number as f64,
            start_column: start_column as f64,
            end_column: end_column as f64,
        }
    }

    let on_editor_created = {
        let memory_curr_instr = props.memory_curr_instr.clone();

        if props.executing {
            memory_curr_instr.set(props.pc);
        }

        use_callback(
            move |editor_link: CodeEditorLink,
                  (memory, memory_text_model, memory_curr_instr, initialized)| {
                editor_link.with_editor(|editor| {
                    let hexdump = memory.generate_formatted_hex(CAPACITY_BYTES);
                    memory_text_model.set_value(&hexdump);

                    let raw_editor = editor.as_ref();
                    let cb_func = &cb.as_ref().unchecked_ref();

                    if *initialized {
                        let coords = get_hex_coords(**memory_curr_instr);
                        raw_editor.on_did_change_cursor_selection(cb_func);
                        raw_editor
                            .reveal_line_in_center(coords.line_number, Some(ScrollType::Smooth));

                        // Highlight line using delta decorations
                        let not_highlighted = js_sys::Array::new();
                        let executed_line = js_sys::Array::new();
                        let decoration: IModelDeltaDecoration =
                            js_sys::Object::new().unchecked_into();
                        let options: IModelDecorationOptions =
                            js_sys::Object::new().unchecked_into();
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
                    }
                });
            },
            (
                props.memory.clone(),
                props.memory_text_model.clone(),
                memory_curr_instr,
                props.initialized,
            ),
        )
    };
    html! {
        <CodeEditor
            classes={"editor"}
            link={editor_link}
            options={get_options()}
            model={memory_text_model.deref().clone()}
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

// ** Helper functions **
// Parse hexdump into a vector of u32 (ready to be stored in memory)
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
