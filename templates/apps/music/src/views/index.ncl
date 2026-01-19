<n:view title="Nucleus Music - Stream the Future">
    <!-- Tailwind for instant premium styling -->
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    colors: {
                        brand: {
                            black: '#141414',
                            red: '#e50914',
                            gray: '#2f2f2f'
                        }
                    },
                    fontFamily: {
                        sans: ['Inter', 'sans-serif'],
                    }
                }
            }
        }
    </script>
    
    <div class="bg-brand-black min-h-screen text-white font-sans selection:bg-brand-red selection:text-white pb-20">
        
        <!-- Navbar (Glassmorphism) -->
        <nav class="fixed w-full z-50 transition-all duration-300 bg-gradient-to-b from-black/80 to-transparent px-8 py-4 flex items-center justify-between">
            <div class="flex items-center gap-12">
                <div class="text-3xl font-bold text-brand-red tracking-tighter">NUCLEUS+</div>
                <div class="hidden md:flex gap-6 text-sm font-medium text-gray-300">
                    <a href="#" class="hover:text-white transition">Home</a>
                    <a href="#" class="text-white">TV Shows</a>
                    <a href="#" class="hover:text-white transition">Movies</a>
                    <a href="#" class="hover:text-white transition">New & Popular</a>
                </div>
            </div>
            <div class="flex items-center gap-6">
                 <button class="hover:text-gray-300"><svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg></button>
                 <div class="w-8 h-8 rounded bg-blue-600 flex items-center justify-center font-bold">N</div>
            </div>
        </nav>

        <!-- Hero Section -->
        <div class="relative h-[85vh] w-full overflow-hidden">
            <!-- Background Image (Simulated) -->
            <div class="absolute inset-0 bg-gray-800 bg-[url('https://images.unsplash.com/photo-1616055192791-d8207908064a?q=80&w=2670&auto=format&fit=crop')] bg-cover bg-center">
                 <div class="absolute inset-0 bg-gradient-to-r from-brand-black via-brand-black/40 to-transparent"></div>
                 <div class="absolute inset-0 bg-gradient-to-t from-brand-black via-transparent to-transparent"></div>
            </div>

            <!-- Content -->
            <div class="relative h-full flex items-center px-8 md:px-16 pt-20">
                <div class="max-w-2xl space-y-6 animate-fade-in-up">
                    <div class="flex items-center gap-2 text-brand-red font-bold tracking-widest text-sm uppercase">
                        <span class="w-1 h-4 bg-brand-red"></span> Series
                    </div>
                    <h1 class="text-6xl md:text-8xl font-black tracking-tighter leading-[0.9]">
                        CYBER<br/>DRIFTER
                    </h1>
                    
                    <div class="flex items-center gap-4 text-green-400 font-bold text-sm">
                        <span>98% Match</span>
                        <span class="text-gray-300">2024</span>
                        <span class="border border-gray-500 rounded px-2 py-0.5 text-gray-300 text-xs">4K Ultra HD</span>
                        <span class="text-gray-300">5.1</span>
                    </div>

                    <p class="text-lg text-gray-300 leading-relaxed font-medium drop-shadow-md max-w-xl">
                        In a future where callbacks are outlawed, one Rust developer must race against the borrow checker to save the digital realm from memory leaks.
                    </p>
                    
                    <div class="flex items-center gap-4 pt-4">
                        <button class="flex items-center gap-3 px-8 py-3 bg-white text-black rounded font-bold hover:bg-white/90 transition text-xl" onclick="playHero()">
                            <span>▶</span> Play
                        </button>
                        <button class="flex items-center gap-3 px-8 py-3 bg-gray-500/40 backdrop-blur-sm text-white rounded font-bold hover:bg-gray-500/50 transition text-xl" onclick="openModalHero()">
                            <span>ℹ</span> More Info
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Trending Section (Simulated Horizontal Scroll) -->
        <div class="px-8 md:px-16 -mt-20 relative z-10 space-y-8">
            <h3 class="text-xl font-bold text-white mb-4">Trending Now</h3>
            <div class="flex gap-4 overflow-x-auto pb-8 scrollbar-hide">
                <div class="flex-shrink-0 w-64 h-36 bg-gray-800 rounded-md hover:scale-105 transition-transform duration-300 cursor-pointer overflow-hidden relative group">
                    <img src="https://images.unsplash.com/photo-1536440136628-849c177e76a1?auto=format&fit=crop&w=400" class="w-full h-full object-cover opacity-80 group-hover:opacity-100 transition">
                    <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition bg-black/50">
                        <div class="w-10 h-10 rounded-full border-2 border-white flex items-center justify-center bg-white/20">▶</div>
                    </div>
                </div>
                <div class="flex-shrink-0 w-64 h-36 bg-gray-800 rounded-md hover:scale-105 transition-transform duration-300 cursor-pointer overflow-hidden relative group">
                    <img src="https://images.unsplash.com/photo-1517604931442-71053643ad4d?auto=format&fit=crop&w=400" class="w-full h-full object-cover opacity-80 group-hover:opacity-100 transition">
                </div>
                <div class="flex-shrink-0 w-64 h-36 bg-gray-800 rounded-md hover:scale-105 transition-transform duration-300 cursor-pointer overflow-hidden relative group">
                    <img src="https://images.unsplash.com/photo-1626814026160-2237a95fc5a0?auto=format&fit=crop&w=400" class="w-full h-full object-cover opacity-80 group-hover:opacity-100 transition">
                </div>
                 <div class="flex-shrink-0 w-64 h-36 bg-gray-800 rounded-md hover:scale-105 transition-transform duration-300 cursor-pointer overflow-hidden relative group">
                    <img src="https://images.unsplash.com/photo-1492684223066-81342ee5ff30?auto=format&fit=crop&w=400" class="w-full h-full object-cover opacity-80 group-hover:opacity-100 transition">
                </div>
            </div>
            
             <h3 class="text-xl font-bold text-white mb-4">Nucleus Originals</h3>
             <div class="flex gap-4 overflow-x-auto pb-4 scrollbar-hide">
                 <!-- Generated placeholders -->
                 <div class="flex-shrink-0 w-[200px] h-[300px] bg-gray-800 rounded mx-1 relative group hover:z-20 hover:scale-110 transition duration-300 origin-center cursor-pointer">
                     <div class="absolute inset-0 bg-gradient-to-t from-black to-transparent z-10"></div>
                     <span class="absolute bottom-4 left-4 z-20 font-bold text-2xl tracking-tighter drop-shadow-lg">DARK<br/>MATTER</span>
                 </div>
                 <div class="flex-shrink-0 w-[200px] h-[300px] bg-gray-700 rounded mx-1 relative group hover:z-20 hover:scale-110 transition duration-300 origin-center cursor-pointer"></div>
                 <div class="flex-shrink-0 w-[200px] h-[300px] bg-gray-600 rounded mx-1 relative group hover:z-20 hover:scale-110 transition duration-300 origin-center cursor-pointer"></div>
             </div>
        </div>
    </div>

    <!-- Scripts -->
    <script src="/static/js/app.js"></script>
    <script type="module">
        import init from '/pkg/nucleus_music.js';
        init();
    </script>
    <script lang="ts">
        console.log("Nucleus Music Player Ready");
        
        // Simple Interaction Handlers
        window.playHero = () => alert("Playing Cyber Drifter...");
        window.openModalHero = () => alert("Opening Details...");
    </script>
</n:view>
