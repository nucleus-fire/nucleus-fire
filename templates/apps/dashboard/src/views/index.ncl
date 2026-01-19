<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nucleus Analytics</title>
    <title>Nucleus Analytics</title>
    <link href="/assets/styles.css" rel="stylesheet">

</head>
<body class="min-h-screen bg-background text-white selection:bg-primary selection:text-white">

    <!-- Navbar -->
    <nav class="border-b border-white/10 glass sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex items-center justify-between h-16">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-gradient-to-tr from-primary to-purple-500 flex items-center justify-center font-bold text-sm">N</div>
                    <span class="font-bold text-xl tracking-tight">Nucleus<span class="text-primary">Analytics</span></span>
                </div>
                <div class="flex items-center gap-4 text-sm font-medium text-gray-400">
                    <span class="text-white">Overview</span>
                    <span class="hover:text-white cursor-pointer transition">Reports</span>
                    <span class="hover:text-white cursor-pointer transition">Settings</span>
                    <div class="w-8 h-8 rounded-full bg-surface border border-white/10"></div>
                </div>
            </div>
        </div>
    </nav>

    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        
        <!-- Header -->
        <div class="flex items-end justify-between mb-8">
            <div>
                <h1 class="text-3xl font-bold">Dashboard</h1>
                <p class="text-gray-400 mt-1">Real-time system performance metrics.</p>
            </div>
            <div class="flex gap-2">
                <button class="px-4 py-2 bg-surface hover:bg-white/5 border border-white/10 rounded-lg text-sm font-medium transition">Export</button>
                <button class="px-4 py-2 bg-primary hover:bg-blue-600 text-white rounded-lg text-sm font-medium shadow-lg shadow-blue-500/20 transition">Refresh</button>
            </div>
        </div>

        <!-- Stats Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
            <!-- Stat Card 1 -->
            <div class="glass p-6 rounded-xl hover:border-primary/50 transition duration-300 group">
                <div class="flex justify-between items-start mb-4">
                    <div class="p-2 bg-blue-500/10 rounded-lg text-blue-500 group-hover:bg-blue-500 group-hover:text-white transition">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                    </div>
                    <span class="text-xs font-medium px-2 py-1 bg-green-500/10 text-green-500 rounded-full">+12.5%</span>
                </div>
                <div class="text-gray-400 text-sm font-medium">Requests / Sec</div>
                <div class="text-3xl font-bold mt-1 text-white track-stat" id="rps">7,612</div>
            </div>

            <!-- Stat Card 2 -->
            <div class="glass p-6 rounded-xl hover:border-purple-500/50 transition duration-300 group">
                <div class="flex justify-between items-start mb-4">
                    <div class="p-2 bg-purple-500/10 rounded-lg text-purple-500 group-hover:bg-purple-500 group-hover:text-white transition">
                         <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    </div>
                    <span class="text-xs font-medium px-2 py-1 bg-green-500/10 text-green-500 rounded-full">-2ms</span>
                </div>
                <div class="text-gray-400 text-sm font-medium">Avg Latency</div>
                <div class="text-3xl font-bold mt-1 text-white">0.4<span class="text-lg text-gray-500 ml-1">ms</span></div>
            </div>

             <!-- Stat Card 3 -->
            <div class="glass p-6 rounded-xl hover:border-pink-500/50 transition duration-300 group">
                <div class="flex justify-between items-start mb-4">
                    <div class="p-2 bg-pink-500/10 rounded-lg text-pink-500 group-hover:bg-pink-500 group-hover:text-white transition">
                       <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
                    </div>
                </div>
                <div class="text-gray-400 text-sm font-medium">Active Users</div>
                <div class="text-3xl font-bold mt-1 text-white" id="users">842</div>
            </div>
            
             <!-- Stat Card 4 -->
            <div class="glass p-6 rounded-xl hover:border-orange-500/50 transition duration-300 group">
                <div class="flex justify-between items-start mb-4">
                    <div class="p-2 bg-orange-500/10 rounded-lg text-orange-500 group-hover:bg-orange-500 group-hover:text-white transition">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path></svg>
                    </div>
                    <span class="text-xs font-medium px-2 py-1 bg-green-500/10 text-green-500 rounded-full">Ok</span>
                </div>
                <div class="text-gray-400 text-sm font-medium">Memory Usage</div>
                <div class="text-3xl font-bold mt-1 text-white">24<span class="text-lg text-gray-500 ml-1">MB</span></div>
            </div>
        </div>

        <!-- Charts Section -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- Main Chart -->
            <div class="lg:col-span-2 glass p-6 rounded-xl">
                <div class="flex items-center justify-between mb-6">
                    <h3 class="font-bold text-lg">Traffic Overview</h3>
                    <select class="bg-surface border border-white/10 rounded-lg text-sm px-3 py-1 focus:outline-none focus:border-primary">
                        <option>Last 24 Hours</option>
                        <option>Last 7 Days</option>
                    </select>
                </div>
                
                <!-- CSS-only Bar Chart -->
                <div class="h-64 flex items-end gap-2" id="chart">
                    <!-- Bars generated by JS below to simulate live data -->
                </div>
            </div>

            <!-- Side Panel -->
            <div class="glass p-6 rounded-xl">
                <h3 class="font-bold text-lg mb-6">System Health</h3>
                <div class="space-y-6">
                    <div>
                        <div class="flex justify-between text-sm mb-2">
                            <span class="text-gray-400">CPU Usage</span>
                            <span class="font-bold">42%</span>
                        </div>
                        <div class="w-full bg-surface rounded-full h-2 overflow-hidden">
                            <div class="bg-primary h-2 rounded-full" style="width: 42%"></div>
                        </div>
                    </div>
                     <div>
                        <div class="flex justify-between text-sm mb-2">
                            <span class="text-gray-400">Database Connections</span>
                            <span class="font-bold">85/100</span>
                        </div>
                        <div class="w-full bg-surface rounded-full h-2 overflow-hidden">
                            <div class="bg-purple-500 h-2 rounded-full" style="width: 85%"></div>
                        </div>
                    </div>
                     <div>
                        <div class="flex justify-between text-sm mb-2">
                            <span class="text-gray-400">Cache Hit Rate</span>
                            <span class="font-bold">99.2%</span>
                        </div>
                        <div class="w-full bg-surface rounded-full h-2 overflow-hidden">
                            <div class="bg-green-500 h-2 rounded-full" style="width: 99%"></div>
                        </div>
                    </div>
                </div>
                
                <div class="mt-8 p-4 bg-blue-500/10 border border-blue-500/20 rounded-lg">
                    <h4 class="text-blue-400 font-bold text-sm mb-1">Atom Reactor Status</h4>
                    <p class="text-xs text-blue-200/70">Runtime optimizations active. LTO enabled. No lock contention detected.</p>
                </div>
            </div>
        </div>
    </main>
    
    <script>
        // Simulate Live Data (Neutron Signal Simulation)
        const minHeight = 10;
        const maxHeight = 100;
        const chart = document.getElementById('chart');
        
        // Init Chart
        for(let i=0; i<30; i++) {
            const h = Math.floor(Math.random() * (maxHeight - minHeight + 1) + minHeight);
            const bar = document.createElement('div');
            bar.className = 'flex-1 bg-surface hover:bg-primary transition-all duration-300 rounded-t-sm relative group cursor-pointer';
            bar.style.height = h + '%';
            
            // Tooltip
            bar.innerHTML = '<div class="absolute -top-10 left-1/2 -translate-x-1/2 bg-black text-white text-xs py-1 px-2 rounded opacity-0 group-hover:opacity-100 pointer-events-none transition">' + h + ' reqs</div>';
            
            chart.appendChild(bar);
        }

        // Live Ticking
        setInterval(() => {
            const rpsEl = document.getElementById('rps');
            let current = parseInt(rpsEl.innerText.replace(',', ''));
            let next = current + Math.floor(Math.random() * 100) - 50;
            rpsEl.innerText = next.toLocaleString();
            
            // Randomly update a bar
            const bars = chart.children;
            const idx = Math.floor(Math.random() * bars.length);
            const h = Math.floor(Math.random() * (maxHeight - minHeight + 1) + minHeight);
            bars[idx].style.height = h + '%';
            bars[idx].innerHTML = '<div class="absolute -top-10 left-1/2 -translate-x-1/2 bg-black text-white text-xs py-1 px-2 rounded opacity-0 group-hover:opacity-100 pointer-events-none transition">' + h + ' reqs</div>';
        }, 2000);
    </script>
</body>
</html>
