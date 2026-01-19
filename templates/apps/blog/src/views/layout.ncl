<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Minimalist Blog</title>
    <link href="/assets/styles.css" rel="stylesheet">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&family=Merriweather:ital,wght@0,300;0,700;1,300&display=swap" rel="stylesheet">
    
    <!-- SEO -->
    <meta name="description" content="A minimalist blog built with Nucleus and Tailwind CSS.">
    <meta property="og:title" content="Minimalist Blog">
    <meta property="og:description" content="A minimalist blog built with Nucleus.">
    <meta property="og:image" content="https://nucleus.dev/og-image.jpg">
    <meta name="twitter:card" content="summary_large_image">

</head>
<body class="bg-white font-serif antialiased text-gray-900 selection:bg-gray-100 selection:text-gray-900">

    <header class="py-10 max-w-4xl mx-auto px-6 flex justify-between items-center border-b border-gray-100">
        <a href="/" class="text-2xl font-bold font-sans tracking-tight hover:text-gray-600 transition">
            My Thoughts
        </a>
        <nav class="space-x-6 text-sm font-sans font-medium">
            <a href="/" class="hover:text-gray-600 transition">Home</a>
            <a href="/about" class="hover:text-gray-600 transition">About</a>
        </nav>
    </header>

    <main class="max-w-4xl mx-auto px-6 py-12 min-h-[60vh]">
        <slot />
    </main>

    <footer class="py-10 text-center text-sm font-sans text-gray-500 border-t border-gray-100 mt-12">
        &copy; 2026 Minimalist Blog using Nucleus
    </footer>

</body>
</html>
