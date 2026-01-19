<n:view title="My Profile">
    <div class="min-h-screen bg-brand-navy font-sans text-white pb-20">
        <!-- Header -->
        <header class="h-16 flex items-center justify-between px-6 border-b border-white/10 bg-brand-navy/80 backdrop-blur sticky top-0 z-50">
            <a href="/app/discover" class="text-gray-400 hover:text-white">
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path></svg>
            </a>
            <h1 class="font-bold text-lg">Edit Profile</h1>
            <button class="text-brand-rose font-medium text-sm">Save</button>
        </header>

        <!-- Content -->
        <main class="max-w-2xl mx-auto p-6 space-y-8 animate-fade-in">
            <!-- Photos -->
            <section class="space-y-4">
                <h2 class="text-sm font-bold text-gray-500 uppercase tracking-wider">Photos</h2>
                <div class="grid grid-cols-3 gap-4">
                    <div class="aspect-[2/3] bg-white/10 rounded-xl overflow-hidden relative group cursor-pointer border border-white/10 hover:border-brand-rose/50 transition-colors">
                        <img src="https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400&q=80" class="w-full h-full object-cover" alt="My Profile Photo 1">
                        <div class="absolute bottom-2 right-2 bg-black/50 p-1 rounded-full text-white cursor-pointer hover:bg-red-500 transition-colors">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                        </div>
                    </div>
                    <div class="aspect-[2/3] bg-white/5 rounded-xl border-2 border-dashed border-white/20 flex flex-col items-center justify-center text-gray-500 hover:border-brand-rose/50 hover:text-brand-rose hover:bg-brand-rose/5 transition-all cursor-pointer">
                        <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
                        <span class="text-xs font-medium">Add Photo</span>
                    </div>
                    <div class="aspect-[2/3] bg-white/5 rounded-xl border-2 border-dashed border-white/20 flex flex-col items-center justify-center text-gray-500 hover:border-brand-rose/50 hover:text-brand-rose hover:bg-brand-rose/5 transition-all cursor-pointer">
                        <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
                        <span class="text-xs font-medium">Add Photo</span>
                    </div>
                </div>
            </section>

            <!-- Bio -->
            <section class="space-y-4">
                 <h2 class="text-sm font-bold text-gray-500 uppercase tracking-wider">About Me</h2>
                 <div class="relative">
                     <textarea class="w-full bg-white/5 border border-white/10 rounded-xl p-4 text-white placeholder-gray-500 focus:outline-none focus:border-brand-rose focus:ring-1 focus:ring-brand-rose transition-all resize-none h-32" placeholder="Write something about yourself..."></textarea>
                     <span class="absolute bottom-4 right-4 text-xs text-gray-500">0/500</span>
                 </div>
            </section>
            
            <!-- Details -->
            <section class="space-y-4">
                <h2 class="text-sm font-bold text-gray-500 uppercase tracking-wider">Details</h2>
                <div class="space-y-4">
                     <div>
                        <label class="block text-xs text-gray-400 mb-1">Job Title</label>
                        <input type="text" value="Product Designer" aria-label="Job Title" class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-brand-rose transition-all">
                    </div>
                    <div>
                        <label class="block text-xs text-gray-400 mb-1">Company</label>
                        <input type="text" value="Nucleus Inc." aria-label="Company" class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-brand-rose transition-all">
                    </div>
                     <div>
                        <label class="block text-xs text-gray-400 mb-1">School</label>
                        <input type="text" placeholder="Add school" aria-label="School" class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-brand-rose transition-all">
                    </div>
                </div>
            </section>
        </main>
    </div>
</n:view>
