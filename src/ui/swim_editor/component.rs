use std::{cell::RefCell, rc::Rc};

use monaco::{
    api::TextModel,
    sys::{
        editor::{
            IEditorMinimapOptions, IEditorScrollbarOptions, IModelDecorationOptions,
            IModelDeltaDecoration, IStandaloneEditorConstructionOptions, ISuggestOptions,
            ScrollType,
        },
        IMarkdownString, Range,
    },
    yew::{CodeEditor, CodeEditorLink},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Callback, Properties};
use yew_hooks::prelude::*;

use crate::{
    agent::datapath_communicator::DatapathCommunicator, emulation_core::architectures::AvailableDatapaths, parser::parser_structs_and_enums::ProgramInfo, ui::{
        assembled_view::component::{DataSegment, TextSegment},
        footer::component::FooterTabState,
    }
};

#[derive(PartialEq, Properties)]
pub struct SwimEditorProps {
    pub text_model: UseStateHandle<TextModel>,
    pub lines_content: Rc<RefCell<Vec<String>>>,
    pub program_info: ProgramInfo,
    pub binary: Vec<u32>,
    pub pc: u64,
    pub pc_limit: usize,
    pub memory_curr_instr: UseStateHandle<u64>,
    pub editor_curr_line: UseStateHandle<f64>,
    pub editor_active_tab: UseStateHandle<EditorTabState>,
    pub console_active_tab: UseStateHandle<FooterTabState>,
    pub current_architecture: AvailableDatapaths,
    pub communicator: &'static DatapathCommunicator
}

#[derive(Default, PartialEq)]
pub enum EditorTabState {
    #[default]
    Editor,
    TextSegment,
    DataSegment,
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
    let editor_active_tab = &props.editor_active_tab;
    let console_active_tab = &props.console_active_tab;
    let speed_class = use_state(|| "valid".to_string());

    // Setup the array that would store hover decorations applied to the
    // text model and initialize the options for it.
    let hover_jsarray = js_sys::Array::new();
    let hover_decor_array = use_mut_ref(js_sys::Array::new);

    let on_editor_created = {
        let curr_line = props.editor_curr_line.clone();
        let lines_content = Rc::clone(&props.lines_content);

        use_callback(
            move |editor_link: CodeEditorLink, curr_line| {
                match editor_link.with_editor(|editor| {
                    let raw_editor = editor.as_ref();
                    let model = raw_editor.get_model().unwrap();
                    // store each line from the original code editor's contents for assembled view
                    let line_count = model.get_line_count() as usize;
                    let mut lines_content = lines_content.borrow_mut();
                    let mut lines = Vec::new();
                    for i in 1..line_count {
                        lines.push(model.get_line_content(i as f64));
                    }
                    *lines_content = lines;
                    // Scroll to current line
                    raw_editor.reveal_line_in_center(**curr_line, Some(ScrollType::Smooth));
                    // Highlight current line using delta decorations
                    let not_highlighted = js_sys::Array::new();
                    let executed_line = js_sys::Array::new();
                    let decoration: IModelDeltaDecoration = js_sys::Object::new().unchecked_into();
                    let options: IModelDecorationOptions = js_sys::Object::new().unchecked_into();
                    if **curr_line != 0.0 {
                        // Show highlight if current line is not 0
                        options.set_inline_class_name("executedLine".into());
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
                    Some(()) => log::debug!("Swim Editor linked!"),
                    None => log::debug!("No swim editor :<"),
                };
            },
            curr_line,
        )
    };

    let change_tab = {
        let editor_active_tab = editor_active_tab.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();
            let tab_name = target
                .get_attribute("label")
                .unwrap_or(String::from("editor"));

            let new_tab: EditorTabState = match tab_name.as_str() {
                "editor" => EditorTabState::Editor,
                "text" => EditorTabState::TextSegment,
                "data" => EditorTabState::DataSegment,
                _ => EditorTabState::default(),
            };

            editor_active_tab.set(new_tab);
        })
    };

    let change_architecture = {
        let communicator = props.communicator;
        Callback::from(move |event: Event| {
            let target = event.target();
            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
            let architecture = input.value();
            let new_architecture: AvailableDatapaths = match architecture.as_str() {
                "mips" => AvailableDatapaths::MIPS,
                "riscv" => AvailableDatapaths::RISCV,
                _ => AvailableDatapaths::MIPS,
            };
            communicator.set_core(new_architecture.clone());
            log::debug!("New architecture: {:?}", new_architecture);
        })
    };

    let change_execution_speed = {
        let communicator = props.communicator;
        Callback::from(move |event: Event| {
            let target = event.target();
            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
            let speed = input.value().parse::<u32>().unwrap_or(1);
            if speed > 0 {
                log::debug!("New execution speed: {:?}", speed);
                communicator.set_execute_speed(speed);
                input.set_class_name("valid");
            } else {
                // get element by id and make it red
                input.set_class_name("invalid");
            }
        })
    };

    // Copies text to the user's clipboard
    let on_clipboard_clicked = {
        let text_model = text_model.clone();
        let clipboard = use_clipboard();
        Callback::from(move |_: _| {
            let text_model = text_model.clone();
            clipboard.write_text(text_model.get_value());
            gloo::dialogs::alert("Your code is saved to the clipboard.\nPaste it onto a text file to save it.\n(Ctrl/Cmd + V)");
        })
    };

    // We'll have the Mouse Hover event running at all times.
    {
        let text_model = text_model.clone();
        let program_info = props.program_info.clone();
        use_event_with_window("mouseover", move |_: MouseEvent| {
            let hover_jsarray = hover_jsarray.clone();
            let hover_decor_array = hover_decor_array.clone();
            let text_model = text_model.clone();
            let curr_model = text_model.as_ref();

            // Parse output from parser and create an instance of IModelDeltaDecoration for each line.
            for (line_number, line_information) in program_info.monaco_line_info.iter().enumerate()
            {
                let decoration: IModelDeltaDecoration = js_sys::Object::new().unchecked_into();

                let hover_range = monaco::sys::Range::new(
                    (line_number + 1) as f64,
                    0.0,
                    (line_number + 1) as f64,
                    0.0,
                );
                let hover_range_js = hover_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                decoration.set_range(&monaco::sys::IRange::from(hover_range_js));

                let hover_opts: IModelDecorationOptions = js_sys::Object::new().unchecked_into();
                hover_opts.set_is_whole_line(true.into());
                let hover_message: IMarkdownString = js_sys::Object::new().unchecked_into();
                js_sys::Reflect::set(
                    &hover_message,
                    &JsValue::from_str("value"),
                    &JsValue::from_str(&line_information.mouse_hover_string),
                )
                .unwrap();
                hover_opts.set_hover_message(&hover_message);
                decoration.set_options(&hover_opts);
                let hover_js = decoration
                    .dyn_into::<JsValue>()
                    .expect("Hover is not found.");
                hover_jsarray.push(&hover_js);
            }

            // log!("This is the array after the push");
            // log!(hover_jsarray.clone());

            // properly pass the handlers onto the array
            let new_hover_decor_array =
                curr_model.delta_decorations(&hover_decor_array.borrow_mut(), &hover_jsarray, None);
            *hover_decor_array.borrow_mut() = new_hover_decor_array;

            // log!("These are the arrays after calling Delta Decorations");
            // log!(hover_jsarray.clone());
            // log!(hover_decor_array.borrow_mut().clone());

            // empty out the array that hold the decorations
            hover_jsarray.set_length(0);

            // log!("These are the arrays after calling popping the hover_jsarray");
            // log!(hover_jsarray.clone());
            // log!(hover_decor_array.borrow_mut().clone());
        });
    };

    let conditional_class = if **editor_active_tab == EditorTabState::Editor {
        ""
    } else {
        "hidden"
    };
    html! {
        <>
            // Editor buttons
            <div class="editor-toolbar">
                <div class="bar tabs">
                if **editor_active_tab == EditorTabState::Editor {
                    <button class={classes!("tab", "pressed")} label="editor" onclick={change_tab.clone()}>{"Editor"}</button>
                } else {
                    <button class="tab" label="editor" onclick={change_tab.clone()}>{"Editor"}</button>
                }

                if **editor_active_tab == EditorTabState::TextSegment {
                    <button class={classes!("tab", "pressed")} label="text" onclick={change_tab.clone()}>{"Text Segment"}</button>
                } else {
                    <button class="tab" label="text" onclick={change_tab.clone()}>{"Text Segment"}</button>
                }

                if **editor_active_tab == EditorTabState::DataSegment {
                    <button class={classes!("tab", "pressed")} label="data" onclick={change_tab.clone()}>{"Data Segment"}</button>
                } else {
                    <button class="tab" label="data" onclick={change_tab.clone()}>{"Data Segment"}</button>
                }
                </div>
                <div class="bar emulator-options">
                    <button class={classes!("copy-button", conditional_class)} title="Copy to Clipboard" onclick={on_clipboard_clicked}>{"Copy to Clipboard "}<i class={classes!("fa-regular", "fa-copy")}></i></button>
                    <input type="number" id="execution-speed" title="Execution Speed" name="execution-speed" placeholder="1" min="1" onchange={change_execution_speed} />
                    <span title="Execution Speed">{"Hz"}</span>
                    <select class="architecture-selector" name="architecture" onchange={change_architecture.clone()} value={
                        match props.current_architecture {
                            AvailableDatapaths::RISCV => "riscv",
                            AvailableDatapaths::MIPS => "mips",
                        }
                    }>
                        <option value="riscv">{"RISC-V"}</option>
                        <option value="mips">{"MIPS"}</option>
                    </select>
                </div>
            </div>
            if **editor_active_tab == EditorTabState::Editor {
                <CodeEditor classes={"editor"} link={link} options={get_options()} model={text_model.clone()} on_editor_created={on_editor_created}/>
            } else if **editor_active_tab == EditorTabState::TextSegment {
                <TextSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} pc={props.pc} editor_active_tab={editor_active_tab.clone()} console_active_tab={console_active_tab.clone()} memory_curr_instr={props.memory_curr_instr.clone()} editor_curr_line={props.editor_curr_line.clone()} communicator={props.communicator}/>
            } else if **editor_active_tab == EditorTabState::DataSegment {
                <DataSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} binary={props.binary.clone()} editor_active_tab={editor_active_tab.clone()} console_active_tab={console_active_tab.clone()} memory_curr_instr={props.memory_curr_instr.clone()} editor_curr_line={props.editor_curr_line.clone()} pc_limit={props.pc_limit}/>
            }
        </>
    }
}
