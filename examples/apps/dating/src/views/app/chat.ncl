<n:view title="Messages">
    <div class="h-screen bg-brand-navy flex overflow-hidden font-sans">
        <!-- Sidebar -->
        <aside class="w-80 border-r border-white/10 flex flex-col glass z-10">
            <div class="h-16 flex items-center justify-between px-6 border-b border-white/10">
                <h1 class="text-white font-bold text-lg">Matches</h1>
                <a href="/app/discover" class="text-brand-rose text-sm font-medium hover:text-white">Discover</a>
            </div>
            
            <div class="flex-1 overflow-y-auto p-4 space-y-2">
                <!-- Active Chat -->
                <div class="p-3 rounded-xl bg-white/10 border border-white/5 cursor-pointer flex items-center gap-3">
                    <div class="relative">
                        <img src="https://images.unsplash.com/photo-1517841905240-472988babdf9?w=100&q=80" class="w-12 h-12 rounded-full object-cover" alt="Sarah's Avatar">
                        <div class="absolute bottom-0 right-0 w-3 h-3 bg-green-500 rounded-full border-2 border-brand-navy"></div>
                    </div>
                    <div class="flex-1 min-w-0">
                        <h3 class="text-white font-medium truncate">Sarah</h3>
                        <p class="text-gray-400 text-sm truncate">See you at 8! ☕️</p>
                    </div>
                    <span class="text-xs text-gray-500">2m</span>
                </div>
                
                <!-- Inactive Chat -->
                <div class="p-3 rounded-xl hover:bg-white/5 cursor-pointer flex items-center gap-3 transition-colors">
                    <img src="https://images.unsplash.com/photo-1539571696357-5a69c17a67c6?w=100&q=80" class="w-12 h-12 rounded-full object-cover grayscale opacity-70" alt="Michael's Avatar">
                    <div class="flex-1 min-w-0">
                        <h3 class="text-gray-300 font-medium truncate">Michael</h3>
                        <p class="text-gray-500 text-sm truncate">Matched on Thursday</p>
                    </div>
                </div>
            </div>
        </aside>

        <!-- Chat Area -->
        <main class="flex-1 flex flex-col relative bg-gradient-to-br from-brand-navy to-black">
            <!-- Header -->
            <header class="h-16 border-b border-white/10 flex items-center justify-between px-6 glass">
                <div class="flex items-center gap-3">
                    <img src="https://images.unsplash.com/photo-1517841905240-472988babdf9?w=100&q=80" class="w-10 h-10 rounded-full object-cover" alt="Sarah Profile">
                    <div>
                        <h2 class="text-white font-bold text-sm">Sarah</h2>
                        <span class="text-green-500 text-xs flex items-center gap-1">● Online</span>
                    </div>
                </div>
                <button class="text-gray-400 hover:text-white">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"></path></svg>
                </button>
            </header>

            <!-- Messages -->
            <div class="flex-1 overflow-y-auto p-6 space-y-6">
                <div class="flex justify-center">
                    <span class="text-xs text-gray-500 bg-white/5 px-3 py-1 rounded-full">Today</span>
                </div>
                
                <div class="flex items-end gap-2">
                    <img src="https://images.unsplash.com/photo-1517841905240-472988babdf9?w=100&q=80" class="w-8 h-8 rounded-full object-cover mb-1" alt="Sarah">
                    <div class="bg-white/10 text-white px-4 py-2 rounded-2xl rounded-bl-none max-w-md">
                        Hey! I noticed you like coffee. Have you tried the new place downtown?
                    </div>
                </div>

                <div class="flex items-end gap-2 flex-row-reverse">
                    <div class="bg-brand-rose text-white px-4 py-2 rounded-2xl rounded-br-none max-w-md">
                        The one on 4th street? I was actually planning to go there today!
                    </div>
                </div>

                <div class="flex items-end gap-2">
                    <img src="https://images.unsplash.com/photo-1517841905240-472988babdf9?w=100&q=80" class="w-8 h-8 rounded-full object-cover mb-1" alt="Sarah">
                    <div class="bg-white/10 text-white px-4 py-2 rounded-2xl rounded-bl-none max-w-md">
                        No way! Want to meet there around 8?
                    </div>
                </div>
                 <div class="flex items-end gap-2 flex-row-reverse">
                    <div class="bg-brand-rose text-white px-4 py-2 rounded-2xl rounded-br-none max-w-md">
                        See you at 8! ☕️
                    </div>
                </div>
            </div>

            <!-- Input -->
            <div class="p-4 border-t border-white/10 glass">
                <form class="flex gap-2">
                    <button type="button" class="p-3 text-gray-400 hover:text-white rounded-full hover:bg-white/5 transition-colors">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                    </button>
                    <input type="text" placeholder="Type a message..." class="flex-1 bg-white/5 border border-white/10 rounded-full px-6 text-white placeholder-gray-500 focus:outline-none focus:border-brand-rose focus:ring-1 focus:ring-brand-rose transition-all">
                    <button type="submit" class="p-3 bg-brand-rose text-white rounded-full hover:bg-brand-rose/90 transition-colors shadow-lg">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path></svg>
                    </button>
                </form>
            </div>
        </main>
    </div>
</n:view>
