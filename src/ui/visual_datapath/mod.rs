//! The visual datapath component.
//!
//! This component operates largely in JS-land, manipulating the page's DOM directly.
//! The visual datapath operates by instantiating an `<object>` element that contains
//! the SVG diagram of the datapath. This offers direct access to its own sub-DOM that
//! can be manipulated, including but not limited to changing the colors of individual
//! lines and components in the datapath.
//!
//! At its core, interactivity is enabled through event listeners. Within the [`VisualDatapath`]
//! struct is a shared reference to a [`Vec<EventListener>`], which contains the currently
//! active event listeners at any given point. Whenever a set of lines are highlighted in
//! the datapath, event listeners are created for each of those lines as well. These event
//! listeners wait for mouse movement over lines to determine when, where, and what
//! information should be displayed.
//!
//! Any existing event listeners need to be disposed of when the diagram updates and a new
//! set of lines are highlighted. This is handled during the `changed()` function built into
//! the Yew framework. After this, it will proceed to re-render the diagram and create new
//! event listeners.
//!
//! Displaying information about a line is done via a "popup" element, which is moved around
//! the page depending on the position of the mouse. Its visibility is controlled by
//! the aforementioned event listeners.

pub mod consts;
pub mod utils;

use std::{cell::RefCell, rc::Rc};

use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Event, HtmlCollection, HtmlElement};
use yew::prelude::*;

use crate::emulation_core::mips::datapath::{MipsDatapath, Stage};
use consts::*;
use utils::*;

#[derive(PartialEq, Properties)]
pub struct VisualDatapathProps {
    pub datapath: MipsDatapath,

    /// A path to the location of the datapath SVG file. This path should be
    /// relative to the project root.
    ///
    /// For example, "`static/datapath.svg`".
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

        if first_render {
            self.initialize(current_stage);
        } else {
            let result = Self::highlight_stage(&get_g_elements(), current_stage);

            // Capture new event listeners.
            if let Ok(new_listeners) = result {
                self.add_event_listeners(new_listeners);
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.clear_event_listeners();

        true
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.clear_event_listeners();
    }
}

impl VisualDatapath {
    /// Move the contents of `new_listeners` to `self.event_listeners`.
    pub fn add_event_listeners(&mut self, new_listeners: Vec<EventListener>) {
        let active_listeners = Rc::clone(&self.active_listeners);
        let mut active_listeners = (*active_listeners).borrow_mut();

        for l in new_listeners {
            (*active_listeners).push(l);
        }
    }

    /// Clear the contents of `self.event_listeners`.
    pub fn clear_event_listeners(&mut self) {
        let active_listeners = Rc::clone(&self.active_listeners);
        let mut active_listeners = (*active_listeners).borrow_mut();
        (*active_listeners).clear();
    }

    /// Make the SVG element on the page interact-able.
    ///
    /// There are aspects about the SVG element from exporting that initially
    /// make it hard to properly interact with. This makes a few adjustments to
    /// the element to allow this functionality.
    ///
    /// This function is written in the way that it is (encapsulating
    /// the initialization within a [`Callback`]) due to the nature of page
    /// loading.
    ///
    /// While the `<VisualDatapath>` element is finished loading on the page when the
    /// `rendered()` function is called, there is a likely chance that the virtual
    /// DOM within the `<object>` element has *not* yet finished loading. This is
    /// circumvented by creating an event listener on the `<object>` element for the
    /// "load" event, which will guarantee when that virtual DOM is actually ready
    /// to be manipulated.
    pub fn initialize(&mut self, current_stage: String) {
        let on_load = Callback::from(move |_| {
            let nodes = get_g_elements();

            do_over_html_collection(&nodes, |g| {
                if g.has_attribute("data-stage") {
                    let paths = g.children();

                    do_over_html_collection_safe(&paths, |item| {
                        if let Some(path) = item {
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
                    });
                }
            });

            Self::highlight_stage(&nodes, current_stage.clone())
        });

        let active_listeners = Rc::clone(&self.active_listeners);

        // Attach the on load listener.
        let on_load_listener = EventListener::new(&get_datapath_root(), "load", move |event| {
            let result = on_load.emit(event.clone());

            // Capture the new event listeners.
            let mut active_listeners = (*active_listeners).borrow_mut();
            if let Ok(new_listeners) = result {
                for l in new_listeners {
                    (*active_listeners).push(l);
                }
            }
        });

        // Capture the "load" event listener.
        let active_listeners = Rc::clone(&self.active_listeners);
        let mut active_listeners = (*active_listeners).borrow_mut();
        (*active_listeners).push(on_load_listener);
    }

    /// Highlight and enable interactivity for all lines in a given stage.
    ///
    /// Consequently un-highlights for all other lines. (Disabling interactivity
    /// for other lines is done within [`Self::changed()`].)
    ///
    /// Takes as input a reference to an [`HtmlCollection`] that contains all the `<g>` elements
    /// of the SVG diagram.
    ///
    /// Valid strings in the `stage` parameter:
    ///  - instruction_fetch
    ///  - instruction_decode
    ///  - execute
    ///  - memory
    ///  - writeback
    ///
    /// If successful, returns a [`Vec<EventListener>`] containing the new event
    /// listeners generated.
    pub fn highlight_stage(
        nodes: &HtmlCollection,
        stage: String,
    ) -> Result<Vec<EventListener>, JsValue> {
        let mut active_listeners: Vec<EventListener> = vec![];

        do_over_html_collection(nodes, |element| {
            if !element.has_attribute("data-stage") {
                // Do nothing if the line has no defined stage.
            } else if element.get_attribute("data-stage").unwrap() == stage {
                // This is an element we want. Highlight it.
                if let Ok(listeners) = Self::activate_element(element) {
                    for l in listeners {
                        active_listeners.push(l);
                    }
                }
            } else {
                // Not an element we want. Stop highlighting it.
                Self::deactivate_element(element).ok();
            }
        });

        Ok(active_listeners)
    }

    /// Activates a given `<g>` element.
    ///
    /// If successful, returns a [`Vec<EventListener>`] containing the newly created event listeners.
    pub fn activate_element(element: &Element) -> Result<Vec<EventListener>, JsValue> {
        // Callback to execute if a line is hovered over.
        let on_mouseover = Callback::from(move |event: web_sys::Event| {
            let event = event.unchecked_into::<MouseEvent>();

            // Get relevant elements.
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            let popup = get_popup_element();
            let title = popup.query_selector(".title").unwrap().unwrap();

            Self::set_active_hovered(&target).ok();

            // Show popup.
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

        // Callback to execute if the mouse moves while hovered on an element.
        let on_mousemove = Callback::from(move |event: Event| {
            let event = event.unchecked_into::<MouseEvent>();

            let popup = get_popup_element();

            // Move popup.
            popup
                .style()
                .set_property("left", &format!("{}px", event.client_x() + 20))
                .ok();
            popup
                .style()
                .set_property("top", &format!("{}px", event.client_y() + 20))
                .ok();
        });

        // Callback to execute if the mouse stops hovering over an element.
        let on_mouseout = Callback::from(move |event: Event| {
            // Get relevant elements.
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            let popup = get_popup_element();

            Self::set_active_unhovered(&target).ok();

            // Hide popup.
            popup.style().set_property("display", "none").ok();
        });

        let mut active_listeners: Vec<EventListener> = vec![];

        // Within a given <g> tag is its composition of <path> elements building a line.
        // Get the children of this element, and iterate over them.
        let children = element.children();
        for i in 0..children.length() {
            let path = children.item(i).unwrap().unchecked_into::<HtmlElement>();

            Self::set_active_unhovered(&path)?;

            if path.tag_name() == "path" {
                // Attach mouseover listener.
                let on_mouseover = on_mouseover.clone();
                let on_mouseover_listener = EventListener::new(&path, "mouseover", move |event| {
                    on_mouseover.emit(event.clone())
                });

                // Attach mousemove listener.
                let on_mousemove = on_mousemove.clone();
                let on_mousemove_listener = EventListener::new(&path, "mousemove", move |event| {
                    on_mousemove.emit(event.clone())
                });

                // Attach mouseout listener.
                let on_mouseout = on_mouseout.clone();
                let on_mouseout_listener = EventListener::new(&path, "mouseout", move |event| {
                    on_mouseout.emit(event.clone())
                });

                // Save the event listeners.
                active_listeners.push(on_mouseover_listener);
                active_listeners.push(on_mousemove_listener);
                active_listeners.push(on_mouseout_listener);
            }
        }

        Ok(active_listeners)
    }

    /// Deactivates a given `<g>` element.
    pub fn deactivate_element(element: &Element) -> Result<(), JsValue> {
        let children = element.children();

        for i in 0..children.length() {
            let path = children.item(i).unwrap().unchecked_into::<HtmlElement>();

            Self::set_inactive(&path)?;
        }

        Ok(())
    }

    /// Set a line to be inactive.
    pub fn set_inactive(path: &HtmlElement) -> Result<(), JsValue> {
        path.style().set_property("cursor", "auto")?;
        Self::set_color(path, INACTIVE_COLOR)?;

        Ok(())
    }

    /// Set a line to be active and unhovered.
    pub fn set_active_unhovered(path: &HtmlElement) -> Result<(), JsValue> {
        path.style().set_property("cursor", "pointer")?;
        Self::set_color(path, ACTIVE_UNHOVERED_COLOR)?;

        Ok(())
    }

    /// Set a line to be active and hovered.
    pub fn set_active_hovered(path: &HtmlElement) -> Result<(), JsValue> {
        Self::set_color(path, ACTIVE_HOVERED_COLOR)?;

        Ok(())
    }

    /// Set a line's color.
    pub fn set_color(path: &HtmlElement, color: &str) -> Result<(), JsValue> {
        path.set_attribute("stroke", color)?;

        if path.tag_name() == "ellipse" {
            path.set_attribute("fill", color)?;
        }

        Ok(())
    }
}
