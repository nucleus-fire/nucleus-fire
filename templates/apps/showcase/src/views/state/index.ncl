<n:layout name="layout">
    <div class="max-w-4xl mx-auto space-y-12">
        <div>
            <h1 class="text-4xl font-bold mb-4">State Management</h1>
            <p class="text-xl text-gray-400">Nucleus provides Signals (reactive primitives) and Stores (centralized state) that sync between Rust backend and View logic.</p>
        </div>
        
        <div class="glass-panel p-8 bg-gradient-to-br from-brand-primary/5 to-purple-500/5">
            <div class="flex flex-col md:flex-row items-center gap-12">
                <div class="w-full md:w-1/2 space-y-6">
                    <h2 class="text-2xl font-bold">Reactive Counter</h2>
                    <p class="text-gray-400">Clicking the buttons updates the signal, which automatically re-renders the DOM with the new value.</p>
                    
                    <div class="flex items-center gap-6 p-6 bg-black/50 rounded-xl border border-white/10">
                        <button onclick="decrement()" class="w-12 h-12 flex items-center justify-center rounded-full bg-white/10 hover:bg-white/20 transition text-2xl font-bold">-</button>
                        <span id="counter" class="text-5xl font-mono font-bold text-white tabular-nums">0</span>
                         <button onclick="increment()" class="w-12 h-12 flex items-center justify-center rounded-full bg-brand-primary hover:bg-blue-500 transition text-2xl font-bold shadow-lg shadow-blue-500/20">+</button>
                    </div>
                </div>
                
                 <div class="w-full md:w-1/2">
                    <h3 class="font-bold text-sm uppercase tracking-wider text-gray-500 mb-2">Under the Hood</h3>
                    <div class="bg-black/50 p-4 rounded-lg font-mono text-sm border border-white/5 overflow-x-auto">
                        <span class="text-gray-500">// Defines a signal</span><br>
                        <span class="text-purple-400">let</span> (count, set_count) = <span class="text-blue-400">create_signal</span>(0);<br><br>
                        <span class="text-gray-500">// Updates trigger effects</span><br>
                        <span class="text-blue-400">create_effect</span>(|| {<br>
                        &nbsp;&nbsp;console.log(<span class="text-green-300">`Count is ${count()}`</span>);<br>
                        });
                    </div>
                </div>
            </div>
        </div>
        
    </div>
    
    <script>
        // Simple client-side simulation for demo
        let count = 0;
        const el = document.getElementById('counter');
        
        function increment() {
            count++;
            update();
        }
        
        function decrement() {
            count--;
            update();
        }
        
        function update() {
            el.innerText = count;
            // Simulated effect
            el.classList.remove('animate-pulse');
            void el.offsetWidth;
            el.classList.add('animate-pulse');
        }
    </script>
</n:layout>
