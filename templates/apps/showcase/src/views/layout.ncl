<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nucleus Showcase</title>
    <link href="/assets/styles.css" rel="stylesheet">
</head>
<body class="bg-brand-dark text-gray-200 font-sans min-h-screen flex flex-col">

    <!-- Navbar -->
    <nav class="sticky top-0 z-50 glass-panel m-4 mt-4 px-6 py-3 flex items-center justify-between">
        <a href="/" class="flex items-center gap-3 group">
            <div class="w-10 h-10 rounded-lg bg-gradient-to-tr from-brand-primary to-brand-accent flex items-center justify-center text-white font-bold text-xl shadow-lg shadow-brand-primary/20 group-hover:scale-105 transition">N</div>
            <span class="font-bold text-xl tracking-tight">Nucleus<span class="text-brand-primary">Showcase</span></span>
        </a>
        <div class="hidden md:flex items-center gap-2 text-sm font-medium">
            <a href="/" class="nav-link">Home</a>
            <a href="/routing" class="nav-link">Routing</a>
            <a href="/state" class="nav-link">State</a>
            <a href="/forms" class="nav-link">Forms</a>
            <a href="/async" class="nav-link">Async</a>
        </div>
        <a href="https://github.com/nucleus-fire/nucleus-fire" target="_blank" class="p-2 text-gray-400 hover:text-white transition">
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
        </a>
    </nav>
    
    <main class="flex-1 max-w-7xl mx-auto w-full px-4 sm:px-6 lg:px-8 py-8 flex flex-col fade-in">
        <slot />
    </main>

    <footer class="border-t border-white/10 mt-auto">
        <div class="max-w-7xl mx-auto py-8 px-4 sm:px-6 lg:px-8 flex flex-col md:flex-row justify-between items-center gap-4">
            <p class="text-gray-500 text-sm">© 2026 Nucleus Framework. MIT License.</p>
            <div class="flex gap-6 text-sm text-gray-400">
                <a href="#" class="hover:text-white transition">Documentation</a>
                <a href="#" class="hover:text-white transition">GitHub</a>
                <a href="#" class="hover:text-white transition">Discord</a>
            </div>
        </div>
    </footer>

</body>
</html>
