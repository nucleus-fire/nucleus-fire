<n:layout name="layout">
    <div class="max-w-4xl mx-auto space-y-12">
        <!-- Hero -->
        <div>
            <h1 class="text-4xl font-bold mb-4">Routing</h1>
            <p class="text-xl text-gray-400">Nucleus uses file-system based routing. Structure your `controllers/` directory, and URLs are generated automatically.</p>
        </div>
        
        <!-- Feature 1: Dynamic Segments -->
        <div class="glass-panel p-8">
            <h2 class="text-2xl font-bold mb-4 flex items-center gap-3">
                <span class="bg-blue-500/20 text-blue-400 px-3 py-1 rounded text-sm font-mono">/users/:id</span>
                Dynamic Segments
            </h2>
            <p class="text-gray-400 mb-6">Capture URL parameters safely with strong typing.</p>
            
            <div class="grid md:grid-cols-2 gap-8">
                <div class="space-y-4">
                    <h3 class="font-bold text-sm uppercase tracking-wider text-gray-500">Code (Controller)</h3>
                    <div class="bg-black/50 p-4 rounded-lg font-mono text-sm border border-white/5">
                        <span class="text-purple-400">async fn</span> <span class="text-blue-400">show_user</span>(<span class="text-orange-400">id</span>: <span class="text-yellow-400">i32</span>) -> Result<Response> {<br>
                        &nbsp;&nbsp;<span class="text-gray-500">// id is automatically parsed to i32</span><br>
                        &nbsp;&nbsp;<span class="text-purple-400">let</span> user = User::find(id).await?;<br>
                        &nbsp;&nbsp;Ok(render(user))<br>
                        }
                    </div>
                </div>
                
                 <div class="space-y-4">
                    <h3 class="font-bold text-sm uppercase tracking-wider text-gray-500">Live Demo</h3>
                    <div class="flex gap-2">
                         <a href="#" class="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg transition text-sm">/users/1</a>
                         <a href="#" class="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg transition text-sm">/users/42</a>
                         <a href="#" class="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg transition text-sm text-red-300">/users/abc (404)</a>
                    </div>
                </div>
            </div>
        </div>

         <!-- Feature 2: Layouts -->
        <div class="glass-panel p-8">
            <h2 class="text-2xl font-bold mb-4">Nested Layouts</h2>
            <p class="text-gray-400 mb-6">Wrap views in reusable layouts defined in `views/layout.ncl`.</p>
            
            <div class="p-6 border-2 border-dashed border-gray-700 rounded-xl relative">
                <span class="absolute top-2 left-2 text-xs text-gray-500 font-mono">layout.ncl</span>
                <div class="flex justify-between items-center mb-8 px-4 py-2 bg-gray-800 rounded">
                    <span>Navbar</span>
                </div>
                
                <div class="p-6 border-2 border-dashed border-blue-500/30 bg-blue-500/5 rounded-lg relative min-h-[100px] flex items-center justify-center">
                     <span class="absolute top-2 left-2 text-xs text-blue-400 font-mono">View Content (Slot)</span>
                     <p class="text-blue-200">This content is dynamically injected.</p>
                </div>
                
                 <div class="mt-8 px-4 py-2 bg-gray-800 rounded">
                    <span>Footer</span>
                </div>
            </div>
        </div>
    </div>
</n:layout>
