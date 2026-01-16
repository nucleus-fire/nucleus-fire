<n:view title="Like Button">
    <div id="like-hero-btn" class="like-button">
        <button id="btn-hero-like">
            ❤️ <span id="count-hero-like">0</span> Likes
        </button>
    </div>

    <!-- Interactive Island Logic (Client-Side Rust) -->
    <n:client>
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
                }) as Box<dyn FnMut()>);
                
                btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
                cb.forget();
            }
        }
    </n:client>

    <style>
        .like-button button {
            background: rgba(255,255,255,0.1);
            border: 1px solid rgba(255,255,255,0.2);
            color: white;
            padding: 0.5rem 1rem;
            border-radius: 20px;
            cursor: pointer;
            transition: all 0.2s;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }
        .like-button button:hover {
            background: rgba(255,255,255,0.2);
            transform: scale(1.05);
        }
    </style>
</n:view>
