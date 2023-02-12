//! Helpful common functions used for the visual datapath.
//! 
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlObjectElement};

use super::consts::*;

/// Returns an [`HtmlObjectElement`] corresponding to the <object> element in HTML.
pub fn get_datapath_root() -> HtmlObjectElement {
  document()
      .get_element_by_id(DATAPATH_ID)
      .unwrap()
      .dyn_into::<HtmlObjectElement>()
      .unwrap()
}

/// Returns an [`HtmlElement`] corresponding to the <div id="popup"> element in HTML.
pub fn get_popup_element() -> HtmlElement {
  document()
      .get_element_by_id("popup")
      .unwrap()
      .unchecked_into::<HtmlElement>()
}

/// Returns an [`HtmlCollection`] containing all the <g> elements within the SVG diagram.
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

/// Perform some function over an [`HtmlCollection`].
pub fn do_over_html_collection<F>(html_collection: HtmlCollection, function: F)
where
    F: Fn(&Element)
{
    for i in 0..html_collection.length() {
        let element = html_collection.item(i).unwrap();

        function(&element);
    }
}