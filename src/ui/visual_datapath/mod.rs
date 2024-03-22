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
use log::debug;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Event, HtmlCollection, HtmlElement, HtmlInputElement};
use yew::prelude::*;

use consts::*;
use utils::*;

use crate::{agent::datapath_reducer::DatapathReducer, emulation_core::mips::datapath::Stage};

#[derive(PartialEq, Properties)]
pub struct VisualDatapathProps {
    pub datapath_state: UseReducerHandle<DatapathReducer>,

    /// A path to the location of the datapath SVG file. This path should be
    /// relative to the project root.
    ///
    /// For example, "`static/datapath_full.svg`".
    pub svg_path: String,
}

pub struct VisualDatapath {
    active_listeners: Rc<RefCell<Vec<EventListener>>>,
    /// Indicates an instance where the visual datapath should force re-render.
    ///
    /// This can occur if the `svg_path` property of the component changes.
    should_reinitialize: bool,
    size: Rc<RefCell<i32>>,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum DatapathSize {
    Big,
    #[default]
    Small,
}

impl Component for VisualDatapath {
    type Message = ();
    type Properties = VisualDatapathProps;

    fn create(_ctx: &Context<Self>) -> Self {
        VisualDatapath {
            active_listeners: Rc::new(RefCell::new(vec![])),
            should_reinitialize: false,
            size: Rc::new(RefCell::new(50)),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let zoom_in_size = Rc::clone(&self.size);
        let zoom_range_size = Rc::clone(&self.size);
        let zoom_value_size = Rc::clone(&self.size);
        let zoom_out_size = Rc::clone(&self.size);
        html! {
            <div id="datapath-wrapper" class="max-h-[50%] relative">
                <div class="overflow-auto w-full h-full basis-1/2 bg-primary-100 z-10 relative">
                    <object data={ctx.props().svg_path.clone()} type="image/svg+xml" id={DATAPATH_ID} class={classes!("datapath", format!("size-{}", self.size.borrow()))}></object>
                </div>
                <div id="popup">
                    <h1 class="title">{ "[Title]" }</h1>
                    <p class="description">{ "[Description]" }</p>
                    <div class="data">
                        <span class="label">{ "Value:" }</span>
                        <span class="code">{ "[bits]" }</span>
                        <span class="meaning">{ "[base 10]" }</span>
                    </div>
                </div>
                <div class="absolute left-7 bottom-4 z-50 flex items-center gap-4">
                    <button class="zoom-button" onclick={ctx.link().callback(move |_| {
                        let mut size = zoom_out_size.borrow_mut();
                        *size -= 10;
                    })}>
                        <svg class="inline-block w-4 h-4 stroke-w-0 fill-accent-blue-400 icon-minus" viewBox="0 0 32 32">
                            <path d="M0 13v6c0 0.552 0.448 1 1 1h30c0.552 0 1-0.448 1-1v-6c0-0.552-0.448-1-1-1h-30c-0.552 0-1 0.448-1 1z"></path>
                        </svg>
                    </button>
                    <input type="range" min="0" max="200" step="10" value={zoom_value_size.borrow().to_string()} onchange={ctx.link().callback(move |e: Event| {
                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                        let value = target.value().parse::<i32>().unwrap();
                        log::debug!("Value: {:?}", value);
                        let mut size = zoom_range_size.borrow_mut();
                        *size = value;
                    })}/>
                    <button class="zoom-button" onclick={ctx.link().callback(move |_| {
                        let mut size = zoom_in_size.borrow_mut();
                        *size += 10;
                    })}>
                        <svg class="inline-block w-4 h-4 stroke-w-0 fill-accent-blue-400 icon-plus" viewBox="0 0 32 32">
                            <path d="M31 12h-11v-11c0-0.552-0.448-1-1-1h-6c-0.552 0-1 0.448-1 1v11h-11c-0.552 0-1 0.448-1 1v6c0 0.552 0.448 1 1 1h11v11c0 0.552 0.448 1 1 1h6c0.552 0 1-0.448 1-1v-11h11c0.552 0 1-0.448 1-1v-6c0-0.552-0.448-1-1-1z"></path>
                        </svg>
                    </button>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // The diagram views the lines *after* the stage has executed. This is so
        // there is actual data to view. A better way to see this is "when stage X is
        // set on the datapath, highlight the lines for stage Y." The datapath stage
        // tells where it will start the next time "execute" is pressed.
        let current_stage = String::from(match ctx.props().datapath_state.mips.current_stage {
            Stage::InstructionFetch => "writeback",
            Stage::InstructionDecode => "instruction_fetch",
            Stage::Execute => "instruction_decode",
            Stage::Memory => "execute",
            Stage::WriteBack => "memory",
        });

        debug!("Current stage: {:?}", current_stage);
        if first_render || self.should_reinitialize {
            self.initialize(current_stage, ctx.props().datapath_state.clone());
            self.should_reinitialize = false;
        } else {
            let result = Self::highlight_stage(
                &get_g_elements(),
                current_stage,
                &ctx.props().datapath_state.clone(),
            );

            // Capture new event listeners.
            if let Ok(new_listeners) = result {
                self.add_event_listeners(new_listeners);
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        self.clear_event_listeners();

        // Re-initialize the component if the path name changed.
        if old_props.svg_path != ctx.props().svg_path {
            self.should_reinitialize = true;
        }

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
    pub fn initialize(
        &mut self,
        current_stage: String,
        datapath_state: UseReducerHandle<DatapathReducer>,
    ) {
        let on_load = Callback::from(move |_| {
            let nodes = get_g_elements();

            do_over_html_collection(&nodes, |g| {
                if g.has_attribute("data-stage") {
                    let paths = g.children();

                    do_over_html_collection_safe(&paths, |item| {
                        if let Some(path) = item {
                            // Allow the path to have event listeners.
                            path.set_attribute("pointer-events", "auto").ok();

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
            Self::highlight_stage(&nodes, current_stage.clone(), &datapath_state.clone())
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

    /// Highlight the current stage and enable interactivity for all lines.
    ///
    /// Consequently un-highlights for all other lines not in the current stage.
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
        datapath_state: &UseReducerHandle<DatapathReducer>,
    ) -> Result<Vec<EventListener>, JsValue> {
        let mut active_listeners: Vec<EventListener> = vec![];

        do_over_html_collection(nodes, |element| {
            if element.has_attribute("data-stage") {
                let is_in_current_stage = element.get_attribute("data-stage").unwrap() == stage;
                if let Ok(listeners) =
                    Self::enable_interactivity(element, datapath_state, is_in_current_stage)
                {
                    for l in listeners {
                        active_listeners.push(l);
                    }
                }
            }
        });

        Ok(active_listeners)
    }

    /// Enables interactivity for a given `<g>` element. This includes both the popup
    /// and coloring functionality.
    ///
    /// `is_active` indicates whether a given element is in the current stage.
    ///
    /// If successful, returns a [`Vec<EventListener>`] containing the newly created event listeners.
    pub fn enable_interactivity(
        element: &Element,
        datapath_state: &UseReducerHandle<DatapathReducer>,
        is_active: bool,
    ) -> Result<Vec<EventListener>, JsValue> {
        // Color a line when hovering.
        let color_on_mouseover = Callback::from(move |event: Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            Self::set_active_hovered(&target).ok();
        });

        // Color a line (a different color) when no longer hovering.
        let color_on_mouseout = Callback::from(move |event: Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            if is_active {
                Self::set_active_unhovered(&target).ok();
            } else {
                Self::set_inactive(&target).ok();
            }
        });

        // Move the popup if the mouse moves while still hovering.
        let popup_on_mousemove = Callback::from(move |event: Event| {
            let event = event.unchecked_into::<MouseEvent>();

            let popup = get_popup_element();

            // Calculate popup position.
            let mouse_position = get_datapath_iframe_mouse_position(event);
            let popup_size = (popup.offset_width(), popup.offset_height());
            let popup_position =
                calculate_popup_position(mouse_position, popup_size, get_window_size());

            // Move popup.
            popup
                .style()
                .set_property("left", &format!("{}px", popup_position.0))
                .ok();
            popup
                .style()
                .set_property("top", &format!("{}px", popup_position.1))
                .ok();
        });

        // Hide the popup when no longer hovering.
        let popup_on_mouseout = Callback::from(move |_| {
            let popup = get_popup_element();
            popup.style().set_property("display", "none").ok();
        });

        let mut active_listeners: Vec<EventListener> = vec![];

        // Within a given <g> tag is its composition of <path> elements building a line.
        // Get the children of this element, and iterate over them.
        let children = element.children();
        for i in 0..children.length() {
            let path = children.item(i).unwrap().unchecked_into::<HtmlElement>();

            // Set the initial path color.
            if is_active {
                Self::set_active_unhovered(&path)?;
            } else {
                Self::set_inactive(&path)?;
            }
            path.style().set_property("stroke-width", "10").unwrap();

            if path.tag_name() == "path" {
                // Set the initial state of this path:
                // Make the mouse look like a hand when the path is hovered.
                path.style().set_property("cursor", "pointer")?;

                // Attach all the event listeners.
                let color_on_mouseover = color_on_mouseover.clone();
                let color_on_mouseover_listener =
                    EventListener::new(&path, "mouseover", move |event| {
                        color_on_mouseover.emit(event.clone())
                    });

                let color_on_mouseout = color_on_mouseout.clone();
                let color_on_mouseout_listener =
                    EventListener::new(&path, "mouseout", move |event| {
                        color_on_mouseout.emit(event.clone())
                    });

                let datapath_state_ref = datapath_state.clone();
                let popup_on_mouseover_listener =
                    EventListener::new(&path, "mouseover", move |event| {
                        let event = event.clone().unchecked_into::<MouseEvent>();
                        // Get relevant elements.
                        let target = event.target().unwrap().unchecked_into::<HtmlElement>();
                        let popup = get_popup_element();

                        // Show popup.
                        let variable = target
                            .parent_element()
                            .unwrap()
                            .get_attribute("data-variable")
                            .unwrap_or_default();
                        populate_popup_information(&datapath_state_ref, &variable);

                        // Calculate popup position.
                        let mouse_position = get_datapath_iframe_mouse_position(event);
                        let popup_size = (popup.offset_width(), popup.offset_height());
                        let popup_position =
                            calculate_popup_position(mouse_position, popup_size, get_window_size());

                        popup
                            .style()
                            .set_property("left", &format!("{}px", popup_position.0))
                            .ok();
                        popup
                            .style()
                            .set_property("top", &format!("{}px", popup_position.1))
                            .ok();
                        popup.style().set_property("display", "block").ok();
                    });

                let popup_on_mousemove = popup_on_mousemove.clone();
                let popup_on_mousemove_listener =
                    EventListener::new(&path, "mousemove", move |event| {
                        popup_on_mousemove.emit(event.clone())
                    });

                let popup_on_mouseout = popup_on_mouseout.clone();
                let popup_on_mouseout_listener =
                    EventListener::new(&path, "mouseout", move |event| {
                        popup_on_mouseout.emit(event.clone())
                    });

                // Save the event listeners.
                active_listeners.push(color_on_mouseover_listener);
                active_listeners.push(color_on_mouseout_listener);
                active_listeners.push(popup_on_mouseover_listener);
                active_listeners.push(popup_on_mousemove_listener);
                active_listeners.push(popup_on_mouseout_listener);
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
        Self::set_color(path, INACTIVE_COLOR)?;

        Ok(())
    }

    /// Set a line to be active and unhovered.
    pub fn set_active_unhovered(path: &HtmlElement) -> Result<(), JsValue> {
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
