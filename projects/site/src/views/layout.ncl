<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nucleus | The Hyper-Optimized Framework</title>
    <meta name="description" content="Build high-performance full-stack apps with Rust. Zero-compromise speed, developer experience, and safety.">
    
    <!-- DNS Prefetch & Preconnect for External Resources -->
    <link rel="dns-prefetch" href="//fonts.googleapis.com">
    <link rel="dns-prefetch" href="//fonts.gstatic.com">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    
    <!-- Favicon -->
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>⚡</text></svg>">
    
    <!-- Critical CSS (inlined for render performance) -->
    <style critical>
        /* Critical above-the-fold styles */
        body { margin: 0; background: #000; color: #ededed; font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif; }
        .nav { position: fixed; top: 0; left: 0; right: 0; z-index: 100; padding: 16px 0; background: rgba(0,0,0,0.8); backdrop-filter: blur(20px); }
    </style>
    
    <!-- Main Stylesheet (non-blocking with media swap) -->
    <link href="/assets/output.css" rel="stylesheet">
    
    <!-- Accessibility Styles (WCAG 2.1 AA) -->
    <link href="/assets/accessibility.css" rel="stylesheet">
    
    <!-- Highlight.js for syntax highlighting (Local) -->
    <link rel="stylesheet" href="/assets/highlight.css?v=4">
    <script src="/assets/highlight.min.js?v=4"></script>

    <!-- Local Fonts (Inter Variable) -->
    <style>
        @font-face {
            font-family: 'Inter';
            font-weight: 100 900;
            font-display: swap;
            font-style: normal;
            font-named-instance: 'Regular';
            src: url("/assets/fonts/Inter-Variable.woff2") format("woff2");
        }
    </style>
</head>
<body>
    <!-- Skip Link for Keyboard Navigation (WCAG 2.4.1) -->
    <a href="#main-content" class="skip-link">Skip to main content</a>
    
    <!-- Screen Reader Announcements (WCAG 4.1.3) -->
    <div id="a11y-announcer" class="sr-only" aria-live="polite" aria-atomic="true"></div>
    
    <n:slot name="content" />
    
    <!-- Nucleus Client-Side Router for SPA-like navigation -->
    <script src="/assets/router.js?v=4" defer></script>
    
    <!-- Docs Script -->
    <script src="/assets/docs.js?v=9" defer></script>
</body>
</html>
