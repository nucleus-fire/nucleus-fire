/**
 * Nucleus Client-Side Router
 * 
 * Features:
 * - Intercepts n:link clicks for SPA-like navigation
 * - Prefetches pages on hover/touch
 * - Uses View Transitions API for smooth animations
 * - Handles browser back/forward with popstate
 * - Falls back gracefully when JS disabled
 * 
 * @version 1.0.0
 * @license MIT
 */
(function() {
  'use strict';

  // Prevent multiple initialization
  if (window.__NUCLEUS_ROUTER_INITIALIZED__) {
    return;
  }
  window.__NUCLEUS_ROUTER_INITIALIZED__ = true;

  // Cache for prefetched pages
  const pageCache = new Map();
  const prefetchingUrls = new Set();
  
  // Track executed scripts to avoid re-execution
  const executedScripts = new Set();
  
  // Config
  const PREFETCH_DELAY = 65; // ms before prefetching on hover
  const CACHE_TTL = 60000;   // 1 minute cache TTL

  /**
   * Initialize router
   */
  function init() {
    // Click handler for nucleus links
    document.addEventListener('click', handleClick);
    
    // Prefetch on hover/focus
    document.addEventListener('mouseover', handleHover);
    document.addEventListener('touchstart', handleHover, { passive: true });
    document.addEventListener('focus', handleHover, true);
    
    // Browser back/forward
    window.addEventListener('popstate', handlePopState);
    
    // IntersectionObserver for viewport prefetching (optional, for data-prefetch="viewport")
    setupViewportPrefetch();
  }
  


  /**
   * Handle click on nucleus links
   */
  function handleClick(e) {
    const link = e.target.closest('[data-nucleus-link]');
    if (!link) return;
    
    // Skip if modifier keys (open in new tab)
    if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
    
    // Skip external links
    if (link.origin !== location.origin) return;
    
    // Skip downloads and non-GET
    if (link.hasAttribute('download')) return;
    
    e.preventDefault();
    navigate(link.href);
  }

  /**
   * Handle hover for prefetching
   */
  let prefetchTimeout = null;
  function handleHover(e) {
    const link = e.target.closest('[data-nucleus-link]');
    if (!link || link.origin !== location.origin) return;
    
    // Cancel previous timeout
    if (prefetchTimeout) clearTimeout(prefetchTimeout);
    
    // Delay prefetch to avoid wasted requests on quick hovers
    prefetchTimeout = setTimeout(() => {
      prefetch(link.href);
    }, PREFETCH_DELAY);
  }

  /**
   * Prefetch a URL
   */
  async function prefetch(url) {
    // Normalize URL
    const normalizedUrl = new URL(url, location.origin).href;
    
    // Skip if already cached or prefetching
    if (pageCache.has(normalizedUrl) || prefetchingUrls.has(normalizedUrl)) return;
    
    prefetchingUrls.add(normalizedUrl);
    
    try {
      const res = await fetch(normalizedUrl, {
        headers: { 'X-Nucleus-Prefetch': 'true' },
        priority: 'low'
      });
      
      if (res.ok) {
        const html = await res.text();
        pageCache.set(normalizedUrl, {
          html,
          timestamp: Date.now()
        });
      }
    } catch {
      // Prefetch failed, silently ignore
    } finally {
      prefetchingUrls.delete(normalizedUrl);
    }
  }

  /**
   * Navigate to a URL
   */
  async function navigate(url, pushState = true) {
    const normalizedUrl = new URL(url, location.origin).href;
    
    // Use View Transitions API if available
    if (document.startViewTransition) {
      document.startViewTransition(async () => {
        await loadPage(normalizedUrl);
      });
    } else {
      await loadPage(normalizedUrl);
    }
    
    if (pushState) {
      history.pushState({ nucleus: true, url: normalizedUrl }, '', normalizedUrl);
    }
    
    // Scroll to top or hash
    const hash = new URL(normalizedUrl).hash;
    if (hash) {
      // Use getElementById which handles numeric IDs correctly (unlike querySelector)
      // Decode the hash to handle encoded characters
      const id = decodeURIComponent(hash.slice(1));
      const el = document.getElementById(id);
      if (el) el.scrollIntoView({ behavior: 'smooth' });
    } else {
      window.scrollTo({ top: 0, behavior: 'instant' });
    }
  }

  /**
   * Load page content
   */
  async function loadPage(url) {
    const normalizedUrl = new URL(url, location.origin).href;
    
    // Check cache first
    const cached = pageCache.get(normalizedUrl);
    if (cached && (Date.now() - cached.timestamp) < CACHE_TTL) {
      applyPage(cached.html);
      return;
    }
    
    try {
      const res = await fetch(normalizedUrl, {
        headers: { 'X-Nucleus-Navigate': 'true' }
      });
      
      if (!res.ok) {
        // Fallback to full navigation on error
        location.href = normalizedUrl;
        return;
      }
      
      const html = await res.text();
      
      // Cache for future
      pageCache.set(normalizedUrl, {
        html,
        timestamp: Date.now()
      });
      
      applyPage(html);
    } catch {
      // Navigation failed, fallback to full page load
      location.href = normalizedUrl;
    }
  }

  /**
   * Apply page content to DOM
   */
  function applyPage(html) {
    // Extract body content
    const bodyMatch = html.match(/<body[^>]*>([\s\S]*)<\/body>/i);
    const bodyContent = bodyMatch ? bodyMatch[1] : html;
    
    // Extract and update title
    const titleMatch = html.match(/<title[^>]*>([\s\S]*?)<\/title>/i);
    if (titleMatch) {
      document.title = titleMatch[1].trim();
    }
    
    // Extract and update meta description
    const descMatch = html.match(/<meta\s+name=["']description["']\s+content=["']([^"']*)["']/i);
    if (descMatch) {
      let meta = document.querySelector('meta[name="description"]');
      if (!meta) {
        meta = document.createElement('meta');
        meta.name = 'description';
        document.head.appendChild(meta);
      }
      meta.content = descMatch[1];
    }
    
    // Swap body content
    document.body.innerHTML = bodyContent;
    
    // Re-run inline scripts, but skip scripts that have already been executed
    // to avoid "Identifier already declared" errors
    document.body.querySelectorAll('script').forEach(oldScript => {
      // Skip external scripts that have already been loaded
      if (oldScript.src) {
        // For external scripts, check if they're already in the page
        const existingScript = document.querySelector(`script[src="${oldScript.src}"]`);
        if (existingScript && existingScript !== oldScript) {
          // Script already loaded, skip re-execution
          oldScript.remove();
          return;
        }
        
        // Skip scripts that define global variables (like docs.js)
        // These should only run once
        if (oldScript.src.includes('docs.js') || 
            oldScript.src.includes('home.js') ||
            oldScript.src.includes('router.js')) {
          oldScript.remove();
          return;
        }
      }
      
      // For inline scripts, only execute page-specific handlers
      // Skip if it looks like it defines global state
      const scriptContent = oldScript.textContent;
      if (scriptContent && (scriptContent.includes('const ') || scriptContent.includes('let ')) &&
          !scriptContent.includes('function()') && !scriptContent.includes('() =>')) {
        // This script defines variables at module scope, risky to re-run
        oldScript.remove();
        return;
      }
      
      const newScript = document.createElement('script');
      Array.from(oldScript.attributes).forEach(attr => {
        newScript.setAttribute(attr.name, attr.value);
      });
      newScript.textContent = oldScript.textContent;
      oldScript.parentNode.replaceChild(newScript, oldScript);
    });
    
    // Dispatch custom event for app re-initialization
    window.dispatchEvent(new CustomEvent('nucleus:navigate', {
      detail: { url: location.href }
    }));
  }

  /**
   * Handle browser back/forward
   */
  function handlePopState(e) {
    if (e.state && e.state.nucleus) {
      navigate(e.state.url || location.href, false);
    } else {
      navigate(location.href, false);
    }
  }

  /**
   * Setup viewport-based prefetching for links with data-prefetch="viewport"
   */
  function setupViewportPrefetch() {
    if (!('IntersectionObserver' in window)) return;
    
    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          prefetch(entry.target.href);
          observer.unobserve(entry.target);
        }
      });
    }, { rootMargin: '50px' });
    
    // Observe links with viewport prefetch
    document.querySelectorAll('[data-nucleus-link][data-prefetch="viewport"]').forEach(link => {
      observer.observe(link);
    });
  }

  // Initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }

  // Expose for manual control
  window.NucleusRouter = {
    navigate,
    prefetch,
    clearCache: () => pageCache.clear()
  };
})();
