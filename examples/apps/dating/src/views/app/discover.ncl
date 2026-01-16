<n:view title="Discover Matches">
    <div class="min-h-screen bg-brand-navy font-sans overflow-hidden flex flex-col">
        <!-- Top Nav -->
        <header class="h-16 flex items-center justify-between px-6 glass z-50">
            <a href="/" class="text-brand-rose font-bold text-xl">Amour</a>
            <div class="flex gap-4">
                <a href="/app/chat" class="p-2 text-gray-400 hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"></path></svg>
                </a>
                <a href="/app/profile" class="p-2 text-gray-400 hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path></svg>
                </a>
            </div>
        </header>

        <!-- Card Stack Container -->
        <main class="flex-1 flex items-center justify-center relative w-full max-w-md mx-auto p-4">
            
            <!-- Background Pulse -->
            <div class="absolute w-[500px] h-[500px] bg-brand-rose/10 blur-[100px] rounded-full -z-10 animate-pulse"></div>

            <!-- Empty State (behind cards) -->
            <div class="absolute inset-0 flex items-center justify-center text-gray-500 z-0">
                <div class="text-center">
                    <p class="mb-4">No more profiles nearby.</p>
                    <button class="px-6 py-2 border border-white/20 rounded-full hover:bg-white/5 transition-colors text-sm">Update Preferences</button>
                </div>
            </div>

            <!-- Profile Card (Top) -->
            <div id="card-stack" class="relative w-full aspect-[3/4] z-10">
                <!-- Mock Data Logic would inject cards here -->
                <div class="absolute inset-0 w-full h-full bg-white rounded-3xl overflow-hidden shadow-2xl transform transition-transform duration-300 hover:scale-[1.02] cursor-grab active:cursor-grabbing">
                    <img src="https://images.unsplash.com/photo-1517841905240-472988babdf9?w=800&q=80" alt="Profile" class="w-full h-full object-cover">
                    <div class="absolute bottom-0 inset-x-0 h-1/2 bg-gradient-to-t from-black/90 via-black/40 to-transparent p-6 flex flex-col justify-end">
                        <div class="flex items-end justify-between">
                            <div>
                                <h2 class="text-3xl font-bold text-white">Sarah, 24</h2>
                                <p class="text-white/80 text-sm mt-1">Interior Designer • 2 miles away</p>
                            </div>
                            <div class="w-10 h-10 rounded-full border border-white/30 flex items-center justify-center text-white/80">
                                ℹ️
                            </div>
                        </div>
                        
                        <div class="flex gap-2 mt-4 flex-wrap">
                            <span class="px-3 py-1 rounded-full bg-white/10 backdrop-blur-sm text-xs text-white border border-white/10">Art</span>
                            <span class="px-3 py-1 rounded-full bg-white/10 backdrop-blur-sm text-xs text-white border border-white/10">Coffee</span>
                            <span class="px-3 py-1 rounded-full bg-white/10 backdrop-blur-sm text-xs text-white border border-white/10">Travel</span>
                        </div>
                    </div>
                </div>
            </div>
        </main>

        <!-- Action Bar -->
        <footer class="h-24 flex items-center justify-center gap-8 pb-4">
            <button class="w-14 h-14 rounded-full bg-gray-800 text-brand-rose border border-gray-700 hover:bg-gray-700 hover:scale-110 transition-all flex items-center justify-center shadow-lg" aria-label="Pass">
                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 18L18 6M6 6l12 12"></path></svg>
            </button>
            <button class="w-16 h-16 rounded-full bg-gradient-to-tr from-brand-rose to-brand-gold text-white hover:scale-110 hover:shadow-brand-rose/50 hover:shadow-xl transition-all flex items-center justify-center shadow-lg" aria-label="Like">
                <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 24 24"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path></svg>
            </button>
            <button class="w-12 h-12 rounded-full bg-gray-800 text-brand-gold border border-gray-700 hover:bg-gray-700 hover:scale-110 transition-all flex items-center justify-center shadow-lg" aria-label="Super Like">
                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"></path></svg>
            </button>
        </footer>
    </div>
</n:view>
