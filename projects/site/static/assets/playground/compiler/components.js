/**
 * Nucleus Component Renderers
 * Transforms NCL component tags into HTML
 * 
 * @module playground/compiler/components
 */

/**
 * Extract attribute value from attribute string
 * @param {string} attrs - Attribute string
 * @param {string} name - Attribute name
 * @returns {string|null} - Attribute value or null
 */
function getAttr(attrs, name) {
    const match = attrs.match(new RegExp(`${name}="([^"]*)"`));
    return match ? match[1] : null;
}

/**
 * Check if attribute exists with value "true"
 * @param {string} attrs - Attribute string
 * @param {string} name - Attribute name
 * @returns {boolean}
 */
function hasAttr(attrs, name) {
    return attrs.includes(`${name}="true"`);
}

/**
 * Render StatCard component
 */
export function renderStatCard(attrs) {
    const value = getAttr(attrs, 'value') || '';
    const label = getAttr(attrs, 'label') || '';
    const trend = getAttr(attrs, 'trend') || '';
    const trendClass = trend.startsWith('+') ? 'text-emerald-500' : 
                       trend.startsWith('-') ? 'text-rose-500' : 'text-gray-500';
    
    return `<div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm">
        <div class="text-gray-500 text-xs mb-1 uppercase font-semibold tracking-wider">${label}</div>
        <div class="text-3xl font-bold text-gray-900">${value}</div>
        ${trend ? `<div class="${trendClass} text-sm mt-2 font-medium">${trend}</div>` : ''}
    </div>`;
}

/**
 * Render FeatureCard component
 */
export function renderFeatureCard(attrs) {
    const icon = getAttr(attrs, 'icon') || '🚀';
    const title = getAttr(attrs, 'title') || '';
    const description = getAttr(attrs, 'description') || '';
    
    return `<div class="p-6 bg-slate-800/50 rounded-2xl border border-slate-700/50">
        <div class="w-12 h-12 bg-indigo-500/20 rounded-xl flex items-center justify-center text-2xl mb-4">${icon}</div>
        <h3 class="text-xl font-bold mb-2 text-white">${title}</h3>
        <p class="text-slate-400">${description}</p>
    </div>`;
}

/**
 * Render Badge component
 */
export function renderBadge(attrs, content) {
    const variant = getAttr(attrs, 'variant') || 'default';
    const icon = getAttr(attrs, 'icon') || '';
    const bgClass = variant === 'primary' ? 'bg-indigo-500/10 text-indigo-400' : 'bg-gray-500/10 text-gray-400';
    return `<span class="inline-block py-1 px-3 rounded-full ${bgClass} text-sm font-medium">${icon ? icon + ' ' : ''}${content}</span>`;
}

/**
 * Render Card component
 */
export function renderCard(attrs, content) {
    const variant = getAttr(attrs, 'variant') || 'default';
    const glass = attrs.includes('glass="true"');
    
    let classes = 'bg-white rounded-xl shadow-sm p-6 border border-gray-100';
    if (variant === 'glass' || glass) {
        classes = 'bg-black/90 backdrop-blur-xl rounded-xl p-6 text-white border border-white/10';
    }
    if (variant === 'feature') {
        classes = 'bg-gradient-to-br from-purple-600 to-blue-600 rounded-xl p-6 text-white';
    }
    
    return `<div class="${classes}">${content}</div>`;
}

/**
 * Render Button component
 */
export function renderButton(attrs, content) {
    const variant = getAttr(attrs, 'variant') || 'primary';
    const size = getAttr(attrs, 'size') || 'md';
    const href = getAttr(attrs, 'href') || '';
    const type = getAttr(attrs, 'type') || 'button';
    const onclick = getAttr(attrs, 'onclick') || '';
    const id = getAttr(attrs, 'id') || '';
    
    const sizeClass = size === 'sm' ? 'px-3 py-1 text-sm' : 
                      size === 'lg' ? 'px-6 py-3 text-lg' : 'px-4 py-2';
    
    const variantClass = {
        primary: 'bg-indigo-600 text-white hover:bg-indigo-700',
        secondary: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50',
        ghost: 'text-gray-600 hover:bg-gray-100',
        gradient: 'bg-gradient-to-r from-indigo-500 to-purple-500 text-white'
    }[variant] || 'bg-indigo-600 text-white hover:bg-indigo-700';
    
    const classes = `${sizeClass} ${variantClass} rounded-lg font-medium transition shadow-sm inline-flex items-center gap-2`;
    const onclickAttr = onclick ? ` onclick="${onclick}"` : '';
    const idAttr = id ? ` id="${id}"` : '';
    
    if (href) {
        return `<a href="${href}"${idAttr} class="${classes}"${onclickAttr}>${content}</a>`;
    }
    return `<button type="${type}"${idAttr} class="${classes}"${onclickAttr}>${content}</button>`;
}

/**
 * Render TextInput component
 */
export function renderTextInput(attrs) {
    const name = getAttr(attrs, 'name') || '';
    const type = getAttr(attrs, 'type') || 'text';
    const label = getAttr(attrs, 'label') || name;
    const placeholder = getAttr(attrs, 'placeholder') || '';
    const required = hasAttr(attrs, 'required') ? 'required' : '';
    const help = getAttr(attrs, 'help') || '';
    const error = getAttr(attrs, 'error') || '';
    const variant = getAttr(attrs, 'variant') || 'default';
    const size = getAttr(attrs, 'size') || 'medium';
    const icon = getAttr(attrs, 'icon') || '';
    const disabled = hasAttr(attrs, 'disabled') ? 'disabled' : '';
    const value = getAttr(attrs, 'value') || '';
    const dependsOn = getAttr(attrs, 'depends_on') || '';
    
    const sizeClasses = {
        small: 'px-3 py-1.5 text-sm',
        medium: 'px-4 py-2',
        large: 'px-5 py-3 text-lg'
    };
    const sizeClass = sizeClasses[size] || sizeClasses.medium;
    
    let inputClasses = `w-full ${sizeClass} focus:ring-2 focus:ring-indigo-500 outline-none transition`;
    if (variant === 'filled') {
        inputClasses += ' bg-gray-100 border-0 rounded-lg focus:bg-white';
    } else if (variant === 'underline') {
        inputClasses += ' border-0 border-b-2 border-gray-300 rounded-none bg-transparent focus:border-indigo-500';
    } else {
        inputClasses += ` border ${error ? 'border-red-300' : 'border-gray-300'} rounded-lg`;
    }
    
    if (disabled) inputClasses += ' bg-gray-100 text-gray-500 cursor-not-allowed';
    
    const dependsAttr = dependsOn ? `data-depends-on="${dependsOn}"` : '';
    const iconHtml = icon ? `<span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">${icon}</span>` : '';
    const inputPadding = icon ? 'pl-10' : '';
    
    return `<div class="mb-4" ${dependsAttr}>
        <label class="block text-sm font-medium mb-1 ${error ? 'text-red-600' : ''}">${label}${required ? ' *' : ''}</label>
        <div class="relative">
            ${iconHtml}
            <input type="${type}" name="${name}" placeholder="${placeholder}" ${required} ${disabled}
                ${value ? `value="${value}"` : ''}
                class="${inputClasses} ${inputPadding}" />
        </div>
        ${help ? `<p class="mt-1 text-sm text-gray-500">${help}</p>` : ''}
        ${error ? `<p class="mt-1 text-sm text-red-600">${error}</p>` : ''}
    </div>`;
}

/**
 * Render Select component
 */
export function renderSelect(attrs, options) {
    const name = getAttr(attrs, 'name') || '';
    const label = getAttr(attrs, 'label') || name;
    const required = hasAttr(attrs, 'required') ? 'required' : '';
    const error = getAttr(attrs, 'error') || '';
    
    const borderClass = error ? 'border-red-300' : 'border-gray-300';
    
    return `<div class="mb-4">
        <label class="block text-sm font-medium mb-1 ${error ? 'text-red-600' : ''}">${label}${required ? ' *' : ''}</label>
        <select name="${name}" ${required} class="w-full px-4 py-2 border ${borderClass} rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none bg-white">
            ${options}
        </select>
        ${error ? `<p class="mt-1 text-sm text-red-600">${error}</p>` : ''}
    </div>`;
}

/**
 * Render Checkbox component
 */
export function renderCheckbox(attrs) {
    const name = getAttr(attrs, 'name') || '';
    const label = getAttr(attrs, 'label') || '';
    const required = hasAttr(attrs, 'required') ? 'required' : '';
    const checked = hasAttr(attrs, 'checked') ? 'checked' : '';
    const variant = getAttr(attrs, 'variant') || 'default';
    
    if (variant === 'toggle') {
        return `<label class="flex items-center gap-3 cursor-pointer mb-4">
            <div class="relative">
                <input type="checkbox" name="${name}" ${required} ${checked} class="sr-only peer" />
                <div class="w-11 h-6 bg-gray-200 rounded-full peer peer-checked:bg-indigo-600 transition"></div>
                <div class="absolute left-1 top-1 w-4 h-4 bg-white rounded-full transition peer-checked:translate-x-5"></div>
            </div>
            <span class="text-sm">${label}</span>
        </label>`;
    }
    
    return `<label class="flex items-center gap-2 mb-4 cursor-pointer">
        <input type="checkbox" name="${name}" ${required} ${checked} class="w-4 h-4 rounded text-indigo-600 focus:ring-indigo-500 border-gray-300" />
        <span class="text-sm">${label}</span>
    </label>`;
}

/**
 * Render FormGroup component
 */
export function renderFormGroup(attrs, content) {
    const legend = getAttr(attrs, 'legend') || '';
    const columns = getAttr(attrs, 'columns') || '1';
    
    return `<fieldset class="mb-6">
        ${legend ? `<legend class="text-lg font-semibold mb-4">${legend}</legend>` : ''}
        <div class="grid grid-cols-${columns} gap-4">${content}</div>
    </fieldset>`;
}

/**
 * Render NavItem component
 */
export function renderNavItem(attrs, content) {
    const href = getAttr(attrs, 'href') || '#';
    const icon = getAttr(attrs, 'icon') || '';
    const active = hasAttr(attrs, 'active');
    const classes = active 
        ? 'flex items-center gap-3 px-4 py-3 bg-indigo-50 text-indigo-700 rounded-lg font-medium'
        : 'flex items-center gap-3 px-4 py-3 text-gray-600 hover:bg-gray-50 rounded-lg font-medium transition';
    return `<a href="${href}" class="${classes}">${icon ? `<span>${icon}</span>` : ''}${content}</a>`;
}

// For backwards compatibility with non-module usage
if (typeof window !== 'undefined') {
    window.NucleusComponents = {
        renderStatCard,
        renderFeatureCard,
        renderBadge,
        renderCard,
        renderButton,
        renderTextInput,
        renderSelect,
        renderCheckbox,
        renderFormGroup,
        renderNavItem
    };
}
