/**
 * HTML Utility Functions
 * 
 * @module playground/utils/html
 */

/**
 * Escape HTML special characters
 * @param {string} text - Text to escape  
 * @returns {string} - Escaped text
 */
export function escapeHTML(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

/**
 * Format HTML with proper indentation
 * @param {string} html - HTML to format
 * @returns {string} - Formatted HTML
 */
export function formatHTML(html) {
    let formatted = '';
    let indent = 0;
    const lines = html.replace(/>\s*</g, '>\n<').split('\n');
    
    lines.forEach(line => {
        line = line.trim();
        if (!line) return;
        
        // Decrease indent for closing tags
        if (line.startsWith('</')) {
            indent = Math.max(0, indent - 1);
        }
        
        formatted += '  '.repeat(indent) + line + '\n';
        
        // Increase indent for opening tags (not self-closing)
        if (line.startsWith('<') && !line.startsWith('</') && 
            !line.endsWith('/>') && !line.includes('</')) {
            indent++;
        }
    });
    
    return formatted.trim();
}

/**
 * Create a debounced version of a function
 * @param {Function} fn - Function to debounce
 * @param {number} delay - Delay in milliseconds
 * @returns {Function} - Debounced function
 */
export function debounce(fn, delay) {
    let timeout;
    return function(...args) {
        clearTimeout(timeout);
        timeout = setTimeout(() => fn.apply(this, args), delay);
    };
}

// For backwards compatibility with non-module usage
if (typeof window !== 'undefined') {
    window.escapeHTML = escapeHTML;
    window.formatHTML = formatHTML;
    window.debounce = debounce;
}
