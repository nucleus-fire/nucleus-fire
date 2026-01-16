/**
 * Toast Notification System
 * 
 * @module playground/utils/toast
 */

// Track active toasts
let toasts = [];

/**
 * Show a toast notification
 * @param {string} message - Toast message
 * @param {string} type - Toast type: 'success', 'error', 'info', 'warning'
 * @param {number} duration - Duration in ms (default 3000)
 */
export function showToast(message, type = 'info', duration = 3000) {
    const toast = { id: Date.now(), message, type };
    toasts.push(toast);
    
    // Create toast element
    const toastEl = document.createElement('div');
    toastEl.className = `toast toast-${type}`;
    toastEl.setAttribute('role', 'alert');
    toastEl.innerHTML = `
        <span class="toast-icon">${getToastIcon(type)}</span>
        <span class="toast-message">${message}</span>
        <button class="toast-close" onclick="this.parentElement.remove()">×</button>
    `;
    
    // Find or create toast container
    let container = document.getElementById('toast-container');
    if (!container) {
        container = document.createElement('div');
        container.id = 'toast-container';
        container.className = 'fixed bottom-4 right-4 z-50 flex flex-col gap-2';
        document.body.appendChild(container);
    }
    
    container.appendChild(toastEl);
    
    // Auto-remove after duration
    setTimeout(() => {
        toastEl.classList.add('toast-exit');
        setTimeout(() => {
            toastEl.remove();
            toasts = toasts.filter(t => t.id !== toast.id);
        }, 300);
    }, duration);
    
    return toast;
}

/**
 * Get icon for toast type
 * @param {string} type - Toast type
 * @returns {string} - Icon emoji
 */
function getToastIcon(type) {
    const icons = {
        success: '✓',
        error: '✕',
        warning: '⚠',
        info: 'ℹ'
    };
    return icons[type] || icons.info;
}

/**
 * Get all active toasts (for testing)
 * @returns {Array} - Array of toast objects
 */
export function getToasts() {
    return toasts;
}

/**
 * Clear all toasts
 */
export function clearToasts() {
    const container = document.getElementById('toast-container');
    if (container) container.innerHTML = '';
    toasts = [];
}

// For backwards compatibility with non-module usage
if (typeof window !== 'undefined') {
    window.showToast = showToast;
    window.getToasts = getToasts;
    window.clearToasts = clearToasts;
}
