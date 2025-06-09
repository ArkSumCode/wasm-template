use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    let msg = "hello world from wasm-template".to_string();
    if let Some(t) = get_element("terminal") {
        t.set_inner_html(&msg);
    }
    msg
}

pub fn get_element(id: &str) -> Option<web_sys::Element> {
    let window = web_sys::window()?;
    let document = window.document()?;
    document.get_element_by_id(id)
}
