/**
 * Nucleus Directive Processors
 * Transforms NCL directives (n:if, n:for, n:form, etc.) into HTML
 * 
 * @module playground/compiler/directives
 */

import { mockData } from '../mockData.js';
import { transpileNeutron } from './neutron.js';

/**
 * Process n:view directive - extract title
 */
export function processView(html) {
    html = html.replace(/<n:view[^>]*title="([^"]*)"[^>]*>/, '<!-- View: $1 -->');
    html = html.replace(/<n:view[^>]*>/, '');
    html = html.replace(/<\/n:view>/g, '');
    return html;
}

/**
 * Process n:layout directive
 */
export function processLayout(html) {
    html = html.replace(/<n:layout\s+name="([^"]*)"[^>]*>/g, '<!-- Layout: $1 -->');
    html = html.replace(/<\/n:layout>/g, '');
    return html;
}

/**
 * Process n:slot directive
 */
export function processSlot(html) {
    html = html.replace(/<n:slot\s*\/>/g, '<!-- slot content -->');
    html = html.replace(/<n:slot\s+name="([^"]*)"\s*\/>/g, '<!-- slot: $1 -->');
    return html;
}

/**
 * Process n:props directive (remove - design-time only)
 */
export function processProps(html) {
    return html.replace(/<n:props>[\s\S]*?<\/n:props>/g, '');
}

/**
 * Process scoped styles
 */
export function processScopedStyles(html) {
    return html.replace(/<style\s+scoped>([\s\S]*?)<\/style>/g, '<style>$1</style>');
}

/**
 * Process n:for loops
 */
export function processFor(html) {
    return html.replace(/<n:for\s+item="(\w+)"\s+in="(\w+)">([\s\S]*?)<\/n:for>/g, 
        (match, item, collection, body) => {
            const data = mockData[collection] || [];
            if (data.length === 0) return `<!-- Loop: ${collection} (empty) -->`;
            
            return data.map(itemData => {
                let result = body;
                result = result.replace(new RegExp(`\\{\\{\\s*${item}\\.(\\w+)\\s*\\}\\}`, 'g'), 
                    (m, prop) => itemData[prop] !== undefined ? itemData[prop] : `[${item}.${prop}]`);
                return result;
            }).join('\n');
        });
}

/**
 * Process {% for %} (Jinja-style) loops
 */
export function processJinjaFor(html) {
    return html.replace(/\{%\s*for\s+(\w+)\s+in\s+(\w+)\s*%\}([\s\S]*?)\{%\s*endfor\s*%\}/g,
        (match, item, collection, body) => {
            const data = mockData[collection] || [{}, {}];
            return data.map((itemData, idx) => {
                let result = body;
                result = result.replace(new RegExp(`\\{\\{\\s*${item}\\.(\\w+)\\s*\\}\\}`, 'g'), 
                    (m, prop) => itemData[prop] !== undefined ? itemData[prop] : `[${item}.${prop}]`);
                return result;
            }).join('\n');
        });
}

/**
 * Process n:if and {% if %} conditionals
 */
export function processIf(html) {
    // n:if
    html = html.replace(/<n:if\s+condition="([^"]*)">([\s\S]*?)<\/n:if>/g, '$2');
    
    // {% if %}
    html = html.replace(/\{%\s*if\s+([\s\S]+?)\s*%\}([\s\S]*?)\{%\s*endif\s*%\}/g, (match, condition, body) => {
        condition = condition.trim();
        if (condition.includes('== 0') || condition.includes('is empty')) {
            return ''; 
        }
        return body;
    });
    
    return html;
}

/**
 * Process inline n:island tags (V3 Syntax)
 */
export function processInlineIsland(html, files) {
    return html.replace(/<n:island([^>]*)>([\s\S]*?)<\/n:island>/g, (match, attrs, content) => {
        try {
            const fakeSource = `use nucleus_std::neutron::*;\n${content}`;
            const { html: islandHtml, script } = transpileNeutron(fakeSource);
            
            const hydrate = attrs.match(/client:(\w+)/)?.[1] || 'load';
            return `<div data-island="inline" data-hydrate="${hydrate}">${islandHtml}<script>${script}</script></div>`;
        } catch (e) {
            console.error('Island compile error:', e);
            return `<div class="p-4 bg-red-50 text-red-600 rounded border border-red-200">Error compiling island: ${e.message}</div>`;
        }
    });
}

/**
 * Process n:island with src attribute (External File Reference)
 */
export function processExternalIsland(html, files) {
    return html.replace(/<n:island\s+src="([^"]*)"([^>]*)\/>/g, (match, src, attrs) => {
        let fileKey = src;
        if (!files[fileKey] && files[fileKey + '.ncl']) fileKey += '.ncl';
        if (!files[fileKey] && files[fileKey + '.rs']) fileKey += '.rs';
        
        const file = files[fileKey];
        if (file && (fileKey.endsWith('.ncl') || fileKey.endsWith('.rs')) && file.content.includes('nucleus_std::neutron')) {
            try {
                const { html: islandsHtml, script } = transpileNeutron(file.content);
                const safeSrc = src.replace(/\//g, '-');
                return `<div data-island="${safeSrc}">${islandsHtml}<script>${script}</script></div>`;
            } catch (e) {
                return `<div class="p-4 bg-red-50 text-red-600 rounded">Error compiling island: ${e.message}</div>`;
            }
        }

        const hydrate = attrs.match(/client:(\w+)/)?.[1];
        const hydrateAttr = hydrate ? ` data-hydrate="${hydrate}"` : '';
        const comment = hydrate ? ` (hydrate: ${hydrate})` : '';
        return `<div data-island="${src}"${hydrateAttr}><!-- Island: ${src}${comment} --></div>`;
    });
}

/**
 * Process n:link directive
 */
export function processLink(html) {
    return html.replace(/<n:link\s+href="([^"]*)"[^>]*>([\s\S]*?)<\/n:link>/g, 
        '<a href="$1" data-prefetch="true">$2</a>');
}

/**
 * Process n:image directive
 */
export function processImage(html) {
    return html.replace(/<n:image\s+src="([^"]*)"[^>]*alt="([^"]*)"[^>]*\/>/g,
        '<img src="$1" alt="$2" loading="lazy" decoding="async" />');
}

/**
 * Process n:model directive (comment only, server-side)
 */
export function processModel(html) {
    html = html.replace(/<n:model[\s\S]*?\/>/g, '<!-- data model binding -->');
    return html;
}

/**
 * Process n:client directive (client-side scripts)
 */
export function processClient(html) {
    html = html.replace(/<n:client>/g, '<script>');
    html = html.replace(/<\/n:client>/g, '</script>');
    return html;
}

/**
 * Process n:script directive (server-side, remove)
 */
export function processScript(html) {
    return html.replace(/<n:script>[\s\S]*?<\/n:script>/g, '');
}

/**
 * Process n:load directive (server data loading, remove)
 */
export function processLoad(html) {
    return html.replace(/<n:load>[\s\S]*?<\/n:load>/g, '');
}

/**
 * Process n:form and n:step directives
 */
export function processForm(html) {
    html = html.replace(/<n:form([^>]*)>/g, (match, attrs) => {
        const action = attrs.match(/action="([^"]*)"/)?.[1] || '';
        const onsubmit = `event.preventDefault(); 
            const formData = Object.fromEntries(new FormData(event.target));
            window.parent.postMessage({ type: 'form:submit', action: '${action}', formData }, '*');`;
        
        return `<form${attrs} onsubmit="${onsubmit.replace(/\n/g, '')}">`;
    });
    html = html.replace(/<\/n:form>/g, '</form>');
    html = html.replace(/<n:step\s+id="([^"]*)"\s+title="([^"]*)">/g, 
        '<fieldset class="wizard-step" data-step="$1"><legend class="text-lg font-semibold mb-4">$2</legend>');
    html = html.replace(/<\/n:step>/g, '</fieldset>');
    return html;
}

/**
 * Process n:field directive
 */
export function processField(html) {
    html = html.replace(/<n:field[^>]*label="([^"]*)"[^>]*>([\s\S]*?)<\/n:field>/g, 
        '<div class="form-group mb-4"><label class="block text-sm font-medium mb-1">$1</label>$2</div>');
    html = html.replace(/<n:field[^>]*label="([^"]*)"[^>]*\/>/g, 
        '<div class="form-group mb-4"><label class="block text-sm font-medium mb-1">$1</label><input class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:ring-2 focus:ring-indigo-500 outline-none"></div>');
    return html;
}

/**
 * Process n:include directive (component imports - show as comments)
 */
export function processInclude(html) {
    return html.replace(/<n:include\s+src="([^"]*)"[^>]*\/>/g, '<!-- Import: $1 -->');
}

// For backwards compatibility with non-module usage
if (typeof window !== 'undefined') {
    window.NucleusDirectives = {
        processView,
        processLayout,
        processSlot,
        processProps,
        processScopedStyles,
        processFor,
        processJinjaFor,
        processIf,
        processInlineIsland,
        processExternalIsland,
        processLink,
        processImage,
        processModel,
        processClient,
        processScript,
        processLoad,
        processForm,
        processField,
        processInclude
    };
}
