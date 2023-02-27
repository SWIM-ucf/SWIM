//! Helpful common functions used for the visual datapath.

use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlObjectElement};

use crate::emulation_core::{datapath::VisualDatapath, mips::datapath::MipsDatapath};

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

/// Returns the relative coordinates of the `<object>` element to the page.
pub fn get_datapath_position() -> (i32, i32) {
    let datapath_root = get_datapath_root();

    (datapath_root.offset_left(), datapath_root.offset_top())
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
pub fn populate_popup_information(datapath: &MipsDatapath, variable: &str) {
    let popup = get_popup_element();

    let title = popup.query_selector(".title").unwrap().unwrap();
    let description = popup.query_selector(".description").unwrap().unwrap();
    let bits = popup.query_selector(".data .code").unwrap().unwrap();
    let meaning = popup.query_selector(".meaning").unwrap().unwrap();

    let information = datapath.visual_line_to_data(variable);

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
