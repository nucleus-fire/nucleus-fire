#![cfg(target_arch = "wasm32")]
#![allow(unused_imports, unused_variables)]
use wasm_bindgen::prelude::*;
use web_sys::{console, WebSocket, MessageEvent, Storage};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

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

        // 1. State & Restoration
        let mut initial_count = 0;
        if let Ok(Some(saved)) = storage.get_item("nucleus_state_count") {
            if let Ok(parsed) = saved.parse::<i32>() {
                console::log_1(&format!("♻️ Restored state: {}", parsed).into());
                initial_count = parsed;
            }
        }
        let count = Rc::new(RefCell::new(initial_count));
        
        let val_span = document.get_element_by_id("count");
        if let Some(span) = val_span {
            span.set_inner_html(&count.borrow().to_string());
            
            // 2. Event Listeners
            let dec_btn = document.get_element_by_id("decrement").expect("dec");
            let count_clone = count.clone();
            let span_clone = span.clone();
            let cb = Closure::wrap(Box::new(move || {
                *count_clone.borrow_mut() -= 1;
                span_clone.set_inner_html(&count_clone.borrow().to_string());
            }) as Box<dyn FnMut()>);
            dec_btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();

            let inc_btn = document.get_element_by_id("increment").expect("inc");
            let count_clone2 = count.clone();
            let span_clone2 = span.clone();
            let cb2 = Closure::wrap(Box::new(move || {
                *count_clone2.borrow_mut() += 1;
                span_clone2.set_inner_html(&count_clone2.borrow().to_string());
            }) as Box<dyn FnMut()>);
            inc_btn.add_event_listener_with_callback("click", cb2.as_ref().unchecked_ref())?;
            cb2.forget();
            
            // 3. HMR State Preservation
            let ws = WebSocket::new("ws://localhost:3000/ws")?;
            let count_for_hmr = count.clone();
            let storage_for_hmr = storage.clone();
            let window_for_hmr = window.clone();
            
            let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                    let msg: String = txt.into();
                    if msg == "reload" {
                         let current = *count_for_hmr.borrow();
                         let _ = storage_for_hmr.set_item("nucleus_state_count", &current.to_string());
                         window_for_hmr.location().reload().unwrap();
                    }
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
        }
    
    }

    Ok(())
}
