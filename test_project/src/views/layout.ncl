<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <meta name="description" content="Built with Nucleus Framework">
    
    <!-- DNS Prefetch & Preconnect (Performance) -->
    <link rel="dns-prefetch" href="//fonts.googleapis.com">
    <link rel="dns-prefetch" href="//fonts.gstatic.com">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    
    <!-- Critical CSS (inlined for render performance) -->
    <style>
        body { margin: 0; font-family: 'Inter', system-ui, sans-serif; background: #111; color: #fff; line-height: 1.5; }
        .container { max-width: 800px; margin: 0 auto; padding: 2rem; }
        header { text-align: center; margin-bottom: 3rem; }
        h1 { font-size: 2.5rem; background: linear-gradient(to right, #4facfe 0%, #00f2fe 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }
        .card { background: #222; padding: 1.5rem; border-radius: 8px; border: 1px solid #333; margin-bottom: 2rem; }
        .features { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
        .feature { background: #1a1a1a; padding: 1rem; border-radius: 6px; }
        code { background: #333; padding: 0.2rem 0.4rem; border-radius: 4px; font-family: monospace; }
    </style>
    
    <!-- Stylesheets -->
    <link href="/assets/style.css" rel="stylesheet">
    
    <!-- Google Fonts (non-render-blocking with media swap) -->
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet" media="print" onload="this.media='all'">
    <noscript><link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet"></noscript>
</head>
<body>
    <n:slot name="content" />
</body>
</html>
