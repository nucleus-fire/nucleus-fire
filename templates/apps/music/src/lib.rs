#![cfg(target_arch = "wasm32")]
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, MessageEvent, Storage, WebSocket};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console::log_1(&"Hydrating Nucleus App (Generalized WASM)...".into());
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let storage = window.session_storage()?.expect("session storage");

    // Common HMR Logic (Optional, can be conditionally added)
    // For now we expose these variables to user blocks.
    {
        {
            let id = "{{ id }}"; // Injected at build time? No, interpolation happens at runtime/compile time.
                                 // Wait, n:client is extracted at compile time. Interpolation inside `n:client` might NOT work if generic.
                                 // BUT `nucleus-cli` logic for `n:client` is raw extraction.
                                 // So we can't interpolate into Rust code easily unless we macro it or use data attributes.

            // Better approach: Select by ID or single class (Demo limitation: Only binds first button)
            let window = web_sys::window().expect("window");
            let document = window.document().expect("document");
            // Use specific ID if possible, but class is fine for now
            if let Some(btn) = document.query_selector(".like-button button").unwrap() {
                let btn = btn.dyn_into::<web_sys::HtmlElement>().unwrap();
                let span = btn.query_selector("span").unwrap().unwrap();
                let initial_count = 0;
                let count = std::rc::Rc::new(std::cell::RefCell::new(initial_count));

                let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    *count.borrow_mut() += 1;
                    span.set_inner_html(&count.borrow().to_string());
                })
                    as Box<dyn FnMut()>);

                btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
                    .unwrap();
                cb.forget();
            }
        }
    }

    Ok(())
}
