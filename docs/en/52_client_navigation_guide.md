# Client-Side Navigation Guide

> Build fast, SPA-like experiences with Nucleus's built-in client-side router.

## Overview

Nucleus includes a powerful client-side router that transforms your multi-page application into a seamless single-page experience. When you use `<n:link>` components, navigation happens **instantly** without full page reloads.

### Features

- ⚡ **Instant Navigation** - AJAX-based page loading with no flicker
- 🎬 **View Transitions** - Smooth animated transitions (Chrome/Edge)
- 📦 **Prefetching** - Pages preload on hover for zero-latency clicks
- 🔙 **History Integration** - Browser back/forward buttons work seamlessly
- 🎯 **Smart Detection** - External links and downloads bypass the router

---

## Quick Start

### Using `<n:link>`

Replace standard `<a>` tags with `<n:link>` to enable client-side navigation:

```html
<!-- Standard HTML link (triggers full page reload) -->
<a href="/about">About</a>

<!-- Nucleus link (client-side navigation) -->
<n:link href="/about">About</n:link>
```

That's it! The router automatically intercepts clicks and handles navigation.

---

## How It Works

### 1. Click Interception

When a user clicks an `<n:link>`:

1. The click event is intercepted
2. Page content is fetched via AJAX
3. The DOM is updated with the new content
4. Browser history is updated
5. A smooth transition animates the change

### 2. Prefetching

By default, pages are **prefetched on hover**:

```html
<!-- Prefetches /dashboard when user hovers -->
<n:link href="/dashboard">Dashboard</n:link>

<!-- Disable prefetch for rarely-visited pages -->
<n:link href="/settings" prefetch="false">Settings</n:link>
```

### 3. View Transitions API

On supported browsers (Chrome 111+, Edge 111+), Nucleus uses the [View Transitions API](https://developer.chrome.com/docs/web-platform/view-transitions/) for smooth animations:

```css
/* Customize the transition */
::view-transition-old(root) {
    animation: fade-out 200ms ease-out;
}

::view-transition-new(root) {
    animation: fade-in 200ms ease-in;
}

@keyframes fade-out {
    from { opacity: 1; }
    to { opacity: 0; }
}

@keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
}
```

---

## Configuration

### Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `href` | String | Required | Target URL |
| `prefetch` | Boolean | `true` | Preload page on hover |
| `replace` | Boolean | `false` | Replace history entry instead of push |
| `class` | String | - | CSS classes to apply |

### Examples

```html
<!-- Basic navigation -->
<n:link href="/products">Products</n:link>

<!-- No prefetch (for heavy pages) -->
<n:link href="/admin/reports" prefetch="false">Reports</n:link>

<!-- Replace history (for redirects) -->
<n:link href="/new-page" replace="true">Continue</n:link>

<!-- With styling -->
<n:link href="/contact" class="btn btn-primary">Contact Us</n:link>
```

---

## External Links

External links are automatically detected and bypass the router:

```html
<!-- Opens in same tab, no router (different origin) -->
<n:link href="https://github.com">GitHub</n:link>

<!-- Opens in new tab -->
<n:link href="https://docs.example.com" target="_blank">External Docs</n:link>
```

### What Bypasses the Router

- Different origin (protocol, host, or port)
- Links with `target="_blank"`
- Links with `download` attribute
- Links to file downloads (`.pdf`, `.zip`, etc.)
- Right-clicks / Cmd+Click / Ctrl+Click (open in new tab)

---

## Programmatic Navigation

Use JavaScript to navigate programmatically:

```javascript
// Navigate to a new page
window.NucleusRouter.navigate('/dashboard');

// Replace current history entry
window.NucleusRouter.navigate('/profile', { replace: true });

// Go back
window.NucleusRouter.back();

// Go forward
window.NucleusRouter.forward();
```

---

## Events

Listen for navigation events:

```javascript
// Before navigation starts
document.addEventListener('nucleus:navigate:start', (e) => {
    console.log('Navigating to:', e.detail.url);
    // Show loading indicator
});

// After navigation completes
document.addEventListener('nucleus:navigate:end', (e) => {
    console.log('Loaded:', e.detail.url);
    // Hide loading indicator
    // Re-initialize any JavaScript
});

// Navigation failed
document.addEventListener('nucleus:navigate:error', (e) => {
    console.error('Navigation failed:', e.detail.error);
});
```

---

## Re-initializing JavaScript

After client-side navigation, you may need to re-initialize page-specific JavaScript:

```javascript
document.addEventListener('nucleus:navigate:end', () => {
    // Re-initialize components
    initializeDropdowns();
    initializeModals();
    
    // Re-run syntax highlighting
    if (window.hljs) {
        hljs.highlightAll();
    }
    
    // Update analytics
    if (window.gtag) {
        gtag('config', 'GA_ID', { page_path: location.pathname });
    }
});
```

---

## Scroll Behavior

By default, navigation scrolls to the top of the page. For hash links, the page scrolls to the element:

```html
<!-- Scrolls to top -->
<n:link href="/about">About</n:link>

<!-- Scrolls to #features section -->
<n:link href="/about#features">Features</n:link>
```

---

## Fallback Behavior

If JavaScript is disabled or an error occurs, `<n:link>` renders as a standard `<a>` tag, ensuring graceful degradation:

```html
<!-- Rendered output -->
<a href="/about" data-n-link>About</a>
```

---

## Best Practices

1. **Use `<n:link>` for internal navigation** - Enables prefetching and transitions
2. **Disable prefetch for heavy pages** - Use `prefetch="false"` for admin panels, reports
3. **Re-initialize JS after navigation** - Listen to `nucleus:navigate:end`
4. **Test without JavaScript** - Ensure links work as fallbacks
5. **Customize transitions** - Use View Transitions API for branded animations

---

## Troubleshooting

### Links not using client-side navigation

- Ensure you're using `<n:link>` not `<a>`
- Check that the link is same-origin
- Verify JavaScript is enabled and no errors in console

### Page content not updating

- Clear browser cache
- Check for JavaScript errors in console
- Ensure the target page returns valid HTML

### Transitions not animating

- View Transitions API requires Chrome/Edge 111+
- Check if `prefers-reduced-motion` is enabled
- Verify no CSS is overriding transitions

---

## See Also

- [Syntax Reference - n:link](#19_syntax_reference)
- [Components Guide](#26_components_guide)
- [Best Practices](#05_best_practices)
