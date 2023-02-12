use std::{cell::RefCell, rc::Rc};

use gloo::utils::document;
use gloo_console::log;
use gloo_events::EventListener;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, Event, HtmlCollection, HtmlElement, HtmlObjectElement};
use yew::prelude::*;

use crate::emulation_core::mips::datapath::{MipsDatapath, Stage};

const DATAPATH_ID: &str = "datapath";

const INACTIVE_COLOR: &str = "#000000";
const ACTIVE_HOVERED_COLOR: &str = "#FF0000";
const ACTIVE_UNHOVERED_COLOR: &str = "#00FFFF";

#[derive(PartialEq, Properties)]
pub struct VisualDatapathProps {
    pub datapath: MipsDatapath,
    pub svg_path: String,
}

pub struct VisualDatapath {
    active_listeners: Rc<RefCell<Vec<EventListener>>>,
}

impl Component for VisualDatapath {
    type Message = ();
    type Properties = VisualDatapathProps;

    fn create(_ctx: &Context<Self>) -> Self {
        VisualDatapath {
            active_listeners: Rc::new(RefCell::new(vec![])),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <object data={ctx.props().svg_path.clone()} type="image/svg+xml" id={DATAPATH_ID}></object>
                <div id="popup">
                    <h1 class="title">{ "[Title]" }</h1>
                    <p class="description">{ "[Description]" }</p>
                    <div class="data">
                        <span class="label">{ "Value:" }</span>
                        <span class="code">{ "[bits]" }</span>
                        <span class="meaning">{ "([base 10] - [register])" }</span>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let current_stage = String::from(match ctx.props().datapath.current_stage {
            Stage::InstructionFetch => "instruction_fetch",
            Stage::InstructionDecode => "instruction_decode",
            Stage::Execute => "execute",
            Stage::Memory => "memory",
            Stage::WriteBack => "writeback",
        });

        log!(&current_stage);

        if first_render {
            self.initialize(current_stage);
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let active_listeners = Rc::clone(&self.active_listeners);
        let active_listeners = (*active_listeners).borrow_mut();

        for listener in &(*active_listeners) {
            drop(listener);
        }
    }
}

impl VisualDatapath {
    // Set a line's color.
    pub fn set_color(path: &HtmlElement, color: &str) -> Result<(), JsValue> {
        path.set_attribute("stroke", color)?;

        if path.tag_name() == "ellipse" {
            path.set_attribute("fill", color)?;
        }

        Ok(())
    }

    // Set a line to be inactive.
    pub fn set_inactive(path: &HtmlElement) -> Result<(), JsValue> {
        path.style().set_property("cursor", "auto")?;
        Self::set_color(path, INACTIVE_COLOR)?;

        Ok(())
    }

    // Set a line to be active and unhovered.
    pub fn set_active_unhovered(path: &HtmlElement) -> Result<(), JsValue> {
        path.style().set_property("cursor", "pointer")?;
        Self::set_color(path, ACTIVE_UNHOVERED_COLOR)?;

        Ok(())
    }

    // Set a line to be active and hovered.
    pub fn set_active_hovered(path: &HtmlElement) -> Result<(), JsValue> {
        Self::set_color(path, ACTIVE_HOVERED_COLOR)?;

        Ok(())
    }

    // Activates a given <g> tag.
    pub fn activate_element(element: &Element) -> Result<Vec<EventListener>, JsValue> {
        let on_mouseover = Callback::from(move |event: web_sys::Event| {
            let event = event.unchecked_into::<MouseEvent>();
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            Self::set_active_hovered(&target).ok();

            // Get popup element
            let popup = document().get_element_by_id("popup").unwrap();
            let popup = popup.unchecked_into::<HtmlElement>();
            let title = popup.query_selector(".title").unwrap().unwrap();

            // Show popup
            let element_id = target.parent_element().unwrap().id();
            title.set_text_content(Some(&element_id));
            popup.style().set_property("display", "block").ok();
            popup
                .style()
                .set_property("left", &format!("{}px", event.client_x() + 20))
                .ok();
            popup
                .style()
                .set_property("top", &format!("{}px", event.client_y() + 20))
                .ok();
        });

        let on_mousemove = Callback::from(move |event: Event| {
            let event = event.unchecked_into::<MouseEvent>();

            // Get popup element
            let popup = document().get_element_by_id("popup").unwrap();
            let popup = popup.unchecked_into::<HtmlElement>();

            // Move popup
            popup
                .style()
                .set_property("left", &format!("{}px", event.client_x() + 20))
                .ok();
            popup
                .style()
                .set_property("top", &format!("{}px", event.client_y() + 20))
                .ok();
        });

        let on_mouseout = Callback::from(move |event: Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            Self::set_active_unhovered(&target).ok();

            // Get popup element
            let popup = document().get_element_by_id("popup").unwrap();
            let popup = popup.unchecked_into::<HtmlElement>();

            // Hide popup
            popup.style().set_property("display", "none").ok();
        });

        let set_active_hovered_event = Closure::<dyn Fn(_)>::new(move |event: web_sys::Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            Self::set_active_hovered(&target).ok();
        });

        let set_active_unhovered_event = Closure::<dyn Fn(_)>::new(move |event: web_sys::Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            Self::set_active_unhovered(&target).ok();
        });

        let children = element.children();

        let mut active_listeners: Vec<EventListener> = vec![];
        for i in 0..children.length() {
            let path = children.item(i).unwrap();
            let path = path.unchecked_into::<HtmlElement>();

            Self::set_active_unhovered(&path)?;

            if path.tag_name() == "path" {
                let on_mouseover = on_mouseover.clone();
                let on_mouseover_listener = EventListener::new(&path, "mouseover", move |event| {
                    on_mouseover.emit(event.clone())
                });

                let on_mousemove = on_mousemove.clone();
                let on_mousemove_listener = EventListener::new(&path, "mousemove", move |event| {
                    on_mousemove.emit(event.clone())
                });

                let on_mouseout = on_mouseout.clone();
                let on_mouseout_listener = EventListener::new(&path, "mouseout", move |event| {
                    on_mouseout.emit(event.clone())
                });

                active_listeners.push(on_mouseover_listener);
                active_listeners.push(on_mousemove_listener);
                active_listeners.push(on_mouseout_listener);
            }
        }
        set_active_hovered_event.forget();
        set_active_unhovered_event.forget();

        Ok(active_listeners)
    }

    // Highlight and enable interactivity for all lines in a given stage.
    // Consequently un-highlights and disables interactivity for all other lines.
    // Valid strings in "stage" parameter:
    //  - instruction_fetch
    //  - instruction_decode
    //  - execute
    //  - memory
    //  - writeback
    pub fn highlight_stage(
        nodes: &HtmlCollection,
        stage: String,
    ) -> Result<Vec<EventListener>, JsValue> {
        let mut active_listeners: Vec<EventListener> = vec![];

        for i in 0..nodes.length() {
            let element = nodes.item(i).unwrap();

            if !element.has_attribute("data-stage") {
                // Do nothing if the line has no defined stage.
            } else if element.get_attribute("data-stage").unwrap() == stage {
                // This is an element we want. Highlight it.
                // log!(&element);

                if let Ok(listeners) = Self::activate_element(&element) {
                    for l in listeners {
                        active_listeners.push(l);
                    }
                }
            }
        }

        Ok(active_listeners)
    }

    pub fn initialize(&mut self, current_stage: String) {
        // Make the SVG interact-able.
        let on_load = Callback::from(move |event: web_sys::Event| {
            let dp = event.target().unwrap();
            let dp = dp.dyn_into::<HtmlObjectElement>().unwrap();

            // Get all the <p> tags.
            let nodes = dp
                .content_document()
                .unwrap()
                .first_element_child()
                .unwrap()
                .query_selector("g")
                .unwrap()
                .unwrap()
                .children();

            for i in 0..nodes.length() {
                let g = nodes.item(i).unwrap();

                if g.has_attribute("data-stage") {
                    let paths = g.children();

                    for j in 0..paths.length() {
                        match paths.item(j) {
                            Some(path) => {
                                // Allow the path to have event listeners.
                                path.set_attribute("pointer-events", "stroke").ok();

                                if path.tag_name() == "ellipse" {
                                    // Remove the large <rect> surrounding the ellipse. It covers up elements and is stupid.
                                    let rects = path
                                        .parent_element()
                                        .unwrap()
                                        .get_elements_by_tag_name("rect");

                                    for k in 0..rects.length() {
                                        let rect = rects.item(k).unwrap();
                                        rect.remove();
                                    }
                                }
                            }
                            None => continue,
                        }
                    }
                }
            }

            Self::highlight_stage(&nodes, current_stage.clone())
        });

        let active_listeners = Rc::clone(&self.active_listeners);

        let dp = document().get_element_by_id(DATAPATH_ID).unwrap();
        let on_load_listener = EventListener::new(&dp, "load", move |event| {
            let mut active_listeners = (*active_listeners).borrow_mut();
            let listeners = on_load.emit(event.clone());
            if let Ok(new_l) = listeners {
                for l in new_l {
                    (*active_listeners).push(l);
                }
            }
        });

        let active_listeners = Rc::clone(&self.active_listeners);
        let mut active_listeners = (*active_listeners).borrow_mut();
        (*active_listeners).push(on_load_listener);
    }
}
