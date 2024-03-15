use std::str::FromStr;
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

use crate::emulation_core::mips::memory::Memory;
use crate::ui::assembled_view::component::StackSegment;
use crate::{
    agent::datapath_communicator::DatapathCommunicator,
    emulation_core::architectures::AvailableDatapaths,
    parser::parser_structs_and_enums::ProgramInfo,
    ui::{
        assembled_view::component::{DataSegment, TextSegment},
        swim_editor::tab::{Tab, TabState},
    },
};
use strum::IntoEnumIterator;

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
    pub editor_active_tab: UseStateHandle<TabState>,
    pub console_active_tab: UseStateHandle<TabState>,
    pub current_architecture: AvailableDatapaths,
    pub speed: u32,
    pub communicator: &'static DatapathCommunicator,
    pub sp: u64,
    pub memory: Memory,
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

            let new_tab: TabState = TabState::from_str(&tab_name).unwrap();
            editor_active_tab.set(new_tab);
        })
    };

    let change_architecture = {
        let communicator = props.communicator;
        Callback::from(move |event: Event| {
            let target = event.target();
            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
            let architecture = input.value();
            let new_architecture: AvailableDatapaths =
                AvailableDatapaths::from(architecture.as_str());
            communicator.set_core(new_architecture.clone());
            log::debug!("New architecture: {:?}", new_architecture);
        })
    };

    let change_execution_speed = {
        let communicator = props.communicator;
        Callback::from(move |event: Event| {
            let target = event.target();
            let input = target.unwrap().unchecked_into::<HtmlInputElement>();
            let speed = input.value().parse::<u32>().unwrap_or(0);
            communicator.set_execute_speed(speed);
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

    let conditional_class = if **editor_active_tab == TabState::Editor {
        ""
    } else {
        "hidden"
    };

    let arch_options = AvailableDatapaths::iter()
        .map(|arch| {
            html! {
                <option value={arch.to_string()} class="bg-primary-700">{arch.to_string()}</option>
            }
        })
        .collect::<Html>();

    html! {
        <>
            // Editor buttons
            <div class="flex flex-row justify-between items-center border-b-2 border-b-solid border-b-primary-200">
                <div class="flex flex-row flex-nowrap min-w-0 items-end h-full">
                    <Tab<TabState> label={TabState::Editor.to_string()} text={"Editor".to_string()} on_click={change_tab.clone()} disabled={false} active_tab={editor_active_tab.clone()} tab_name={TabState::Editor}/>
                    <Tab<TabState> label={TabState::TextSegment.to_string()} text={"Text Segment".to_string()} on_click={change_tab.clone()} disabled={false} active_tab={editor_active_tab.clone()} tab_name={TabState::TextSegment}/>
                    <Tab<TabState> label={TabState::DataSegment.to_string()} text={"Data Segment".to_string()} on_click={change_tab.clone()} disabled={false} active_tab={editor_active_tab.clone()} tab_name={TabState::DataSegment}/>
                    <Tab<TabState> label={TabState::StackSegment.to_string()} text={"Stack Segment".to_string()} on_click={change_tab.clone()} disabled={false} active_tab={editor_active_tab.clone()} tab_name={TabState::StackSegment}/>
                </div>
                <div class="flex flex-row flex-wrap justify-end items-center gap-2 cursor-default">
                    <button class={classes!("copy-button", conditional_class)} title="Copy to Clipboard" onclick={on_clipboard_clicked}>{"Copy to Clipboard "}<i class={classes!("fa-regular", "fa-copy")}></i></button>
                    <input type="number" id="execution-speed" title="Execution Speed. Setting this to 0 will make it run as fast as possible." name="execution-speed" placeholder="0" min="0" value={format!("{}", props.speed)} class="bg-primary-700 flex items-center flex-row text-right w-24" onchange={change_execution_speed} />
                    <span title="Execution Speed.">{"Hz"}</span>
                    <select class="bg-primary-600 flex flex-row items-center" name="architecture" onchange={change_architecture.clone()} value={props.current_architecture.to_string()}>
                        {arch_options}
                    </select>
                </div>
            </div>
            if **editor_active_tab == TabState::Editor {
                <CodeEditor classes={"editor"} link={link} options={get_options()} model={text_model.clone()} on_editor_created={on_editor_created}/>
            } else if **editor_active_tab == TabState::TextSegment {
                <TextSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} pc={props.pc} editor_active_tab={editor_active_tab.clone()} console_active_tab={console_active_tab.clone()} memory_curr_instr={props.memory_curr_instr.clone()} editor_curr_line={props.editor_curr_line.clone()} communicator={props.communicator}/>
            } else if **editor_active_tab == TabState::DataSegment {
                <DataSegment lines_content={props.lines_content.clone()} program_info={props.program_info.clone()} binary={props.binary.clone()} editor_active_tab={editor_active_tab.clone()} console_active_tab={console_active_tab.clone()} memory_curr_instr={props.memory_curr_instr.clone()} editor_curr_line={props.editor_curr_line.clone()} pc_limit={props.pc_limit}/>
            } else if **editor_active_tab == TabState::StackSegment {
                <StackSegment memory_curr_instr={props.memory_curr_instr.clone()} console_active_tab={console_active_tab.clone()} sp={props.sp} memory={props.memory.clone()}/>
            }
        </>
    }
}
