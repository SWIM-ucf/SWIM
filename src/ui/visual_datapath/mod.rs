use gloo::utils::document;
// use gloo_console::log;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlObjectElement};
use yew::prelude::*;

use crate::emulation_core::mips::datapath::MipsDatapath;

const DATAPATH_ID: &str = "datapath";

const INACTIVE_COLOR: &str = "#000000";
const ACTIVE_HOVERED_COLOR: &str = "#FF0000";
const ACTIVE_UNHOVERED_COLOR: &str = "#00FFFF";

#[derive(PartialEq, Properties)]
pub struct VisualDatapathProps {
    pub datapath: MipsDatapath,
    pub svg_path: String,
}

pub struct VisualDatapath;

impl Component for VisualDatapath {
    type Message = ();
    type Properties = VisualDatapathProps;

    fn create(_ctx: &Context<Self>) -> Self {
      VisualDatapath
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <object data={ctx.props().svg_path.clone()} type="image/svg+xml" id={DATAPATH_ID}></object>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            Self::initialize();
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
    pub fn activate_element(element: &Element) -> Result<(), JsValue> {
        let set_active_hovered_event = Closure::<dyn Fn(_)>::new(move |event: web_sys::Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            Self::set_active_hovered(&target).ok();
        });

        let set_active_unhovered_event = Closure::<dyn Fn(_)>::new(move |event: web_sys::Event| {
          let target = event.target().unwrap().unchecked_into::<HtmlElement>();
          Self::set_active_unhovered(&target).ok();
        });

        let children = element.children();

        for i in 0..children.length() {
            let path = children.item(i).unwrap();
            let path = path.unchecked_into::<HtmlElement>();

            Self::set_active_unhovered(&path)?;

            if path.tag_name() == "path" {
                path.add_event_listener_with_callback("mouseover", set_active_hovered_event.as_ref().unchecked_ref()).ok();
                path.add_event_listener_with_callback("mouseout", set_active_unhovered_event.as_ref().unchecked_ref()).ok();

                /*
                path.addEventListener("mouseover", setActiveHoveredEvent);
                path.addEventListener("mouseout", setActiveUnhoveredEvent);
                path.addEventListener("mouseover", openPopupEvent);
                path.addEventListener("mousemove", movePopupEvent);
                path.addEventListener("mouseout", closePopupEvent);
                */
            }
        }
        set_active_hovered_event.forget();
        set_active_unhovered_event.forget();

        Ok(())
    }

    pub fn deactivate_element(element: &Element) -> Result<(), JsValue> {
        let children = element.children();

        for i in 0..children.length() {
            let path = children.item(i).unwrap();
            let path = path.unchecked_into::<HtmlElement>();

            Self::set_inactive(&path)?;

            if path.tag_name() == "path" {
                // path.remove_event_listener_with_callback("mouseover", &events.set_active_hovered_event.as_ref().unchecked_ref());

                /*
                path.removeEventListener("mouseover", setActiveHoveredEvent);
                path.removeEventListener("mouseout", setActiveUnhoveredEvent);
                path.removeEventListener("mouseover", openPopupEvent);
                path.removeEventListener("mousemove", movePopupEvent);
                path.removeEventListener("mouseout", closePopupEvent);
                */
            }
        }

        Ok(())
    }

    // Highlight and enable interactivity for all lines in a given stage.
    // Consequently un-highlights and disables interactivity for all other lines.
    // Valid strings in "stage" parameter:
    //  - instruction_fetch
    //  - instruction_decode
    //  - execute
    //  - memory
    //  - writeback
    pub fn highlight_stage(nodes: &HtmlCollection, stage: &str) -> Result<(), JsValue> {
        for i in 0..nodes.length() {
            let element = nodes.item(i).unwrap();

            if !element.has_attribute("data-stage") {
                // Do nothing if the line has no defined stage.
            } else if element.get_attribute("data-stage").unwrap() == stage {
                // This is an element we want. Highlight it.
                // log!(&element);

                Self::activate_element(&element)?;
            } else {
                // Not an element we want. Stop highlighting it.
                Self::deactivate_element(&element)?;
            }
        }

        Ok(())
    }

    pub fn initialize() {
        // Make the SVG interact-able.
        let pre_process_datapath = Closure::<dyn Fn(_)>::new(move |event: web_sys::Event| {
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

            Self::highlight_stage(&nodes, "instruction_decode").ok();
        });

        let document = document().get_element_by_id(DATAPATH_ID).unwrap();
        document.add_event_listener_with_callback("load", pre_process_datapath.as_ref().unchecked_ref()).ok();
        pre_process_datapath.forget();
    }
}
