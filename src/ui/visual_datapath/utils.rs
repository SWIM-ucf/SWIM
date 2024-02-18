//! Helpful common functions used for the visual datapath.

use gloo::utils::{document, window};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlObjectElement, MouseEvent};
use yew::UseReducerHandle;

use crate::{agent::datapath_reducer::DatapathReducer, emulation_core::{self, architectures::AvailableDatapaths}};

use super::consts::*;

/// Returns an [`HtmlObjectElement`] corresponding to the `<object>` element in HTML.
pub fn get_datapath_root() -> HtmlObjectElement {
    document()
        .get_element_by_id(DATAPATH_ID)
        .unwrap()
        .dyn_into::<HtmlObjectElement>()
        .unwrap()
}

/// Returns an [`HtmlElement`] corresponding to the `<div id="popup">` element in HTML.
pub fn get_popup_element() -> HtmlElement {
    document()
        .get_element_by_id("popup")
        .unwrap()
        .unchecked_into::<HtmlElement>()
}

/// Returns an [`HtmlCollection`] containing all the `<g>` elements within the SVG diagram.
pub fn get_g_elements() -> HtmlCollection {
    get_datapath_root()
        .content_document()
        .unwrap()
        .first_element_child()
        .unwrap()
        .query_selector("g")
        .unwrap()
        .unwrap()
        .children()
}

/// Returns the size of the browser window in pixels.
pub fn get_window_size() -> (i32, i32) {
    (
        window().inner_width().unwrap().as_f64().unwrap() as i32,
        window().inner_height().unwrap().as_f64().unwrap() as i32,
    )
}

/// Returns the relative coordinates of the `<object>` element to the page.
pub fn get_datapath_position() -> (i32, i32) {
    let datapath_root = get_datapath_root();

    (datapath_root.offset_left(), datapath_root.offset_top())
}

/// Given a [`MouseEvent`] inside the datapath `<object>` element, returns
/// the coordinates of the mouse on the full screen.
///
/// This coordinate is highly dependent on the implementation and structure
/// of the page. This should be considered more like a macro than a re-usable
/// function.
pub fn get_datapath_iframe_mouse_position(event: MouseEvent) -> (i32, i32) {
    let datapath_position = get_datapath_position();

    let datapath_wrapper = gloo::utils::document()
        .query_selector(".datapath-wrapper")
        .unwrap()
        .unwrap()
        .unchecked_into::<HtmlElement>();
    let scroll_position = (
        datapath_wrapper.scroll_left(),
        datapath_wrapper.scroll_top(),
    );

    (
        event.client_x() + datapath_position.0 - scroll_position.0,
        event.client_y() + datapath_position.1 - scroll_position.1,
    )
}

/// Given the mouse location, the size of the popup, and the window size,
/// return the coordinates of the top left corner of where the popup should go.
pub fn calculate_popup_position(
    mouse_position: (i32, i32),
    popup_size: (i32, i32),
    window_size: (i32, i32),
) -> (i32, i32) {
    // The horizontal and vertical distance that the popup should be from the mouse.
    const MOUSE_GAP: i32 = 20;

    // As a start, try to put the popup to the lower-right of the mouse.
    let mut position = (mouse_position.0 + MOUSE_GAP, mouse_position.1 + MOUSE_GAP);

    // If the popup gets cut off at the bottom, go to the upper-right of the mouse instead.
    if position.1 + popup_size.1 > window_size.1 {
        position = (position.0, mouse_position.1 - popup_size.1 - MOUSE_GAP);
    }

    // If the popup gets cut off at the right, force the x-position against the side of the screen.
    if position.0 + popup_size.0 > window_size.0 {
        position = (window_size.0 - popup_size.0, position.1);
    }

    position
}

/// Perform some function over an [`HtmlCollection`], assuming each element
/// inside of it is valid.
pub fn do_over_html_collection<F>(html_collection: &HtmlCollection, mut function: F)
where
    F: FnMut(&Element),
{
    for i in 0..html_collection.length() {
        let element = html_collection.item(i).unwrap();

        function(&element);
    }
}

/// Perform some function over an [`HtmlCollection`], but without unwrapping
/// each element.
///
/// This allows the programmer to check first if an `unwrap()` was successful,
/// for example.
pub fn do_over_html_collection_safe<F>(html_collection: &HtmlCollection, mut function: F)
where
    F: FnMut(&Option<Element>),
{
    for i in 0..html_collection.length() {
        let element = html_collection.item(i);

        function(&element);
    }
}

/// Set the data contained in the popup.
///
/// Parameters:
/// - `datapath`: A reference to the datapath that information will be pulled from.
/// - `variable`: The "variable" attribute of the line in the diagram that will have information.
pub fn populate_popup_information(datapath_state: &UseReducerHandle<DatapathReducer>, variable: &str) {
    let popup = get_popup_element();

    let title = popup.query_selector(".title").unwrap().unwrap();
    let description = popup.query_selector(".description").unwrap().unwrap();
    let bits = popup.query_selector(".data .code").unwrap().unwrap();
    let meaning = popup.query_selector(".meaning").unwrap().unwrap();

    let information;
    match datapath_state.current_architecture {
        AvailableDatapaths::MIPS => {
            information = emulation_core::mips::line_info::visual_line_to_data(variable, datapath_state);
        },
        AvailableDatapaths::RISCV => {
            // replace with RISC-V version
            // information = emulation_core::riscv::line_info::visual_line_to_data(variable, datapath_state);
            information = emulation_core::mips::line_info::visual_line_to_data(variable, datapath_state);
        }
    };

    title.set_text_content(Some(&information.title));
    description.set_text_content(Some(&information.description));
    bits.set_text_content(Some(&u64_to_bits(information.value, information.bits)));
    meaning.set_text_content(Some(&u64::to_string(&information.value)));
}

/// Convert an integer value to a string, limited to `bits` number of bits.
///
/// If `bits` is less than 64, the lower `bits` number of bits in `value` will be used.
pub fn u64_to_bits(mut value: u64, bits: u64) -> String {
    let mut output = String::new();

    for _ in 0..bits {
        let bit = (value % 2) as u32;
        output.push(char::from_digit(bit, 10).unwrap_or_default());
        value /= 2;
    }

    output = output.chars().rev().collect::<String>();

    output
}