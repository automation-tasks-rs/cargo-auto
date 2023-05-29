//! src/web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! Trying to isolate/hide all javascript code and conversion here.

// region: use
// the macro unwrap! shows the TRUE location where the error has ocurred.
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
// use wasm_bindgen_futures::JsFuture;
use web_sys::console;
// use web_sys::{Request, RequestInit, Response};
// endregion: use

/// return the global window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// add event listener for button
pub fn add_listener_to_button(element_id: &str, fn_on_click_button: &'static (dyn Fn() + 'static)) {
    let handler_1 = Box::new(move || {
        fn_on_click_button();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    let html_element = get_html_element_by_id(element_id);
    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// set inner text
pub fn set_html_element_inner_text(element_id: &str, inner_text: &str) {
    let html_element = get_html_element_by_id(element_id);
    html_element.set_inner_text(inner_text);
}

/// WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
/// Only correctly html encoded strings can use this function.
/// set inner html into dom
pub fn set_html_element_inner_html(element_id: &str, inner_html: &str) {
    let html_element = get_element_by_id(element_id);
    html_element.set_inner_html(inner_html);
}

// open URL in new tab
pub fn open_url_in_new_tab(url: &str) {
    window().open_with_url_and_target(url, "_blank").unwrap();
}
