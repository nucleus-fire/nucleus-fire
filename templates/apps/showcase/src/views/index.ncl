<n:layout name="layout">
    <div class="flex flex-col items-center text-center py-20 space-y-8">
        <div class="inline-flex items-center px-4 py-2 rounded-full border border-brand-primary/30 bg-brand-primary/10 text-brand-primary text-sm font-medium mb-4 animate-bounce">
            ✨ Introducing Nucleus v1.0
        </div>
        
        <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-white via-blue-100 to-blue-200 pb-2">
            The Full-Stack<br />Framework for Rust
        </h1>
        
        <p class="max-w-2xl text-xl text-gray-400">
            Build type-safe, high-performance web applications with a developer experience similar to Next.js or Rails, but powered by the Atom Reactor.
        </p>
        
        <div class="flex flex-wrap justify-center gap-4 mt-4">
            <a href="/routing" class="px-8 py-4 bg-brand-primary hover:bg-blue-600 text-white font-bold rounded-xl transition shadow-lg shadow-blue-500/25 flex items-center gap-2">
                Explore Features <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path></svg>
            </a>
            <a href="https://nucleusrust.com/docs" target="_blank" class="px-8 py-4 bg-white/5 hover:bg-white/10 border border-white/10 text-white font-medium rounded-xl transition backdrop-blur-sm">
                Read Documentation
            </a>
        </div>
    </div>
    
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-12">
        <a href="/routing" class="glass-panel p-8 hover:border-brand-primary/50 transition group cursor-pointer">
            <div class="p-3 bg-blue-500/10 rounded-lg w-fit mb-4 text-blue-400 group-hover:text-blue-300 transition">
                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
            </div>
            <h3 class="text-xl font-bold mb-2 group-hover:text-brand-primary transition">Smart Routing</h3>
            <p class="text-gray-400">File-system based routing with dynamic segments, layouts, and middleware. Zero configuration required.</p>
        </a>
        
        <a href="/state" class="glass-panel p-8 hover:border-purple-500/50 transition group cursor-pointer">
            <div class="p-3 bg-purple-500/10 rounded-lg w-fit mb-4 text-purple-400 group-hover:text-purple-300 transition">
                 <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path></svg>
            </div>
            <h3 class="text-xl font-bold mb-2 group-hover:text-purple-400 transition">State Management</h3>
            <p class="text-gray-400">Built-in signals and stores. Share state between Rust logic and NCL views seamlessly.</p>
        </a>
        
        <a href="/forms" class="glass-panel p-8 hover:border-pink-500/50 transition group cursor-pointer">
             <div class="p-3 bg-pink-500/10 rounded-lg w-fit mb-4 text-pink-400 group-hover:text-pink-300 transition">
                 <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg>
            </div>
            <h3 class="text-xl font-bold mb-2 group-hover:text-pink-400 transition">Forms & Actions</h3>
            <p class="text-gray-400">Type-safe form handling with automatic validation, CSRF protection, and error mapping.</p>
        </a>
    </div>
</n:layout>
