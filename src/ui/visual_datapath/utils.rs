//! Helpful common functions used for the visual datapath.

use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlObjectElement};

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
