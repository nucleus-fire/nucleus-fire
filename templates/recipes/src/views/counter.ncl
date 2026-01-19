<n:view title="Counter">
    <div style="text-align: center; margin-top: 5rem;">
        <h1>Interactive Counter (WASM)</h1>
        
        <!-- V3: Using Neutron Island + Signals -->
        <n:island client:load>
            <n:script>
                // Modern Signal-based state
                let count = Signal::new(0);
                
                // Persistence logic could be added here via effects if needed
                create_effect(move || {
                    let current = *count.get();
                   // console::log_1(&format!("Count changed: {}", current).into());
                });
            </n:script>

            <div id="counter-app">
                <button onclick={count.update(|c| *c -= 1)} style="padding: 10px 20px; font-size: 1.5rem;">-</button>
                <span style="font-size: 3rem; margin: 0 2rem;">{count}</span>
                <button onclick={count.update(|c| *c += 1)} style="padding: 10px 20px; font-size: 1.5rem;">+</button>
            </div>
        </n:island>
        
        <p style="margin-top: 2rem; color: #888;">This logic runs in your browser via WebAssembly (Neutron).</p>
        <a href="/" style="color: #4facfe;">&larr; Back to Recipes</a>
    </div>
</n:view>
