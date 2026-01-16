<n:view title="Amour - Find True Love">
    <div class="min-h-screen bg-brand-navy text-white font-sans overflow-x-hidden selection:bg-brand-rose selection:text-white">
        
        <!-- Navigation -->
        <nav class="fixed w-full z-50 transition-all duration-300 glass">
            <div class="max-w-7xl mx-auto px-6 h-20 flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <div class="w-8 h-8 rounded-full bg-gradient-to-tr from-brand-rose to-brand-gold"></div>
                    <span class="text-2xl font-bold tracking-tight">Amour</span>
                </div>
                <div class="hidden md:flex items-center gap-8">
                    <a href="#features" class="text-sm font-medium text-gray-300 hover:text-white transition-colors">Stories</a>
                    <a href="#safety" class="text-sm font-medium text-gray-300 hover:text-white transition-colors">Safety</a>
                    <a href="/login" class="px-6 py-2 rounded-full border border-white/20 hover:bg-white/10 transition-all text-sm font-medium">Log in</a>
                </div>
            </div>
        </nav>

        <!-- Hero Section -->
        <main class="relative pt-32 pb-20 lg:pt-48 lg:pb-32 px-6">
            <!-- Ambient Background Glow -->
            <div class="absolute top-0 left-1/2 -translate-x-1/2 w-full h-[800px] bg-brand-rose/20 blur-[120px] rounded-full pointer-events-none -z-10 mix-blend-screen"></div>

            <div class="max-w-4xl mx-auto text-center space-y-8 animate-fade-in">
                <h1 class="text-5xl md:text-7xl font-bold tracking-tight leading-tight">
                    Dating deserving of <br/>
                    <span class="text-transparent bg-clip-text bg-gradient-to-r from-brand-rose to-brand-gold">your standards.</span>
                </h1>
                <p class="text-lg md:text-xl text-gray-400 max-w-2xl mx-auto leading-relaxed">
                    Designed for meaningful connections. No endless swiping, just curated matches 
                    based on what truly matters to you.
                </p>
                
                <div class="flex flex-col sm:flex-row items-center justify-center gap-4 pt-4 animate-slide-up">
                    <a href="/app/discover" class="group relative px-8 py-4 bg-white text-brand-navy rounded-full font-bold text-lg hover:shadow-[0_0_40px_-10px_rgba(255,255,255,0.3)] transition-all">
                        Start Matching
                        <span class="absolute right-4 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 group-hover:translate-x-1 transition-all">→</span>
                    </a>
                    <a href="/about" class="px-8 py-4 rounded-full hover:bg-white/5 transition-colors font-medium text-gray-300">
                        How it works
                    </a>
                </div>
            </div>

            <!-- Stats / Social Proof -->
            <div class="mt-24 grid grid-cols-2 md:grid-cols-4 gap-8 max-w-5xl mx-auto border-t border-white/10 pt-12">
                <div class="text-center">
                    <div class="text-3xl font-bold text-white">2M+</div>
                    <div class="text-sm text-gray-500 mt-1">Daily Matches</div>
                </div>
                <div class="text-center">
                    <div class="text-3xl font-bold text-white">4.9/5</div>
                    <div class="text-sm text-gray-500 mt-1">App Store Rating</div>
                </div>
                <div class="text-center">
                    <div class="text-3xl font-bold text-white">100%</div>
                    <div class="text-sm text-gray-500 mt-1">Verified Profiles</div>
                </div>
                <div class="text-center">
                    <div class="text-3xl font-bold text-white">24/7</div>
                    <div class="text-sm text-gray-500 mt-1">Support Team</div>
                </div>
            </div>
        </main>

        <!-- Features Section -->
        <section id="features" class="py-24 bg-brand-navy/50 relative overflow-hidden">
             <!-- Background Texture -->
             <div class="absolute inset-0 opacity-5 bg-[url('/static/noise.png')]"></div>
             
             <div class="max-w-7xl mx-auto px-6">
                 <div class="text-center mb-16 space-y-4">
                     <h2 class="text-4xl font-bold">More than just a swipe</h2>
                     <p class="text-gray-400 max-w-2xl mx-auto">We've reimagined online dating from the ground up to focus on authenticity.</p>
                 </div>
                 
                 <div class="grid md:grid-cols-3 gap-8">
                     <!-- Feature 1 -->
                     <div class="p-8 rounded-3xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors group">
                         <div class="w-12 h-12 rounded-2xl bg-brand-rose/20 flex items-center justify-center mb-6 text-2xl group-hover:scale-110 transition-transform">🎭</div>
                         <h3 class="text-xl font-bold mb-3">Video Stories</h3>
                         <p class="text-gray-400 leading-relaxed">See the real person behind the profile with 15-second video intros.</p>
                     </div>
                     
                     <!-- Feature 2 -->
                     <div class="p-8 rounded-3xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors group">
                         <div class="w-12 h-12 rounded-2xl bg-brand-gold/20 flex items-center justify-center mb-6 text-2xl group-hover:scale-110 transition-transform">✨</div>
                         <h3 class="text-xl font-bold mb-3">Interest Matching</h3>
                         <p class="text-gray-400 leading-relaxed">Our AI analyzes shared passions to suggest compatible conversation starters.</p>
                     </div>
                     
                     <!-- Feature 3 -->
                     <div class="p-8 rounded-3xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors group">
                         <div class="w-12 h-12 rounded-2xl bg-purple-500/20 flex items-center justify-center mb-6 text-2xl group-hover:scale-110 transition-transform">🔒</div>
                         <h3 class="text-xl font-bold mb-3">Private Mode</h3>
                         <p class="text-gray-400 leading-relaxed">Control who sees your profile. Go incognito anytime.</p>
                     </div>
                 </div>
             </div>
        </section>

        <!-- Safety Section -->
        <section id="safety" class="py-24 relative">
            <div class="max-w-7xl mx-auto px-6 flex flex-col md:flex-row items-center gap-16">
                <div class="md:w-1/2 space-y-6">
                    <h2 class="text-4xl font-bold">Your safety is our priority.</h2>
                    <p class="text-gray-400 text-lg leading-relaxed">
                        We use advanced encryption and strict verification processes to ensure every connection is genuine.
                        <br/><br/>
                        Zero bots. Zero harassment. 100% Amour.
                    </p>
                    <ul class="space-y-4 pt-4">
                        <li class="flex items-center gap-3">
                            <span class="text-green-400">✓</span> Photo Verification
                        </li>
                        <li class="flex items-center gap-3">
                            <span class="text-green-400">✓</span> 24/7 Moderation
                        </li>
                        <li class="flex items-center gap-3">
                            <span class="text-green-400">✓</span> Blocking & Reporting
                        </li>
                    </ul>
                </div>
                <div class="md:w-1/2">
                    <div class="relative aspect-square rounded-[3rem] overflow-hidden border border-white/10 shadow-2xl">
                         <div class="absolute inset-0 bg-gradient-to-br from-brand-rose via-brand-navy to-brand-gold opacity-50"></div>
                         <div class="absolute inset-0 flex items-center justify-center">
                             <span class="text-9xl opacity-20">🛡️</span>
                         </div>
                    </div>
                </div>
            </div>
        </section>
