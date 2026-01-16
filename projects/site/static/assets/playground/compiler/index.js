/**
 * Nucleus NCL Compiler
 * Main entry point for the playground compiler
 * 
 * @module playground/compiler
 */

import { mockData } from '../mockData.js';
import { transpileNeutron } from './neutron.js';
import * as components from './components.js';
import * as directives from './directives.js';

// Demo values for variable interpolation
const demoValues = {
    'stats.total_users': '1,234',
    'stats.revenue': '$45,678',
    'stats.orders': '892',
    'user.name': 'Alice Johnson',
    'user.email': 'alice@example.com',
    'user.avatar': 'https://i.pravatar.cc/40?1',
    'user.role': 'Admin',
    'post.title': 'Getting Started with Nucleus',
    'post.slug': 'getting-started',
    'post.excerpt': 'Learn how to build modern web apps...',
    'post.author': 'John Doe',
    'post.date': 'Jan 8, 2026',
    'post.cover_image': 'https://picsum.photos/400/200'
};

/**
 * Compile NCL source code to HTML
 * 
 * @param {string} ncl - NCL source code
 * @param {string} css - CSS source code
 * @param {Object} files - Project files for island resolution
 * @param {Object} options - Compiler options
 * @returns {string} - Compiled HTML
 */
export function compile(ncl, css = '', files = {}, options = {}) {
    let html = ncl;
    const componentDefs = {};
    
    // 1. Extract component definitions
    html = html.replace(/<n:component\s+name="(\w+)">([\s\S]*?)<\/n:component>/g, (match, name, body) => {
        componentDefs[name] = body;
        return '';
    });
    
    // 2. Process directives
    html = directives.processView(html);
    html = directives.processLayout(html);
    html = directives.processSlot(html);
    html = directives.processProps(html);
    html = directives.processScopedStyles(html);
    html = directives.processFor(html);
    html = directives.processJinjaFor(html);
    html = directives.processIf(html);
    
    // 3. Process islands (before n:script removal)
    html = directives.processInlineIsland(html, files);
    html = directives.processExternalIsland(html, files);
    
    // 4. Process remaining directives
    html = directives.processLink(html);
    html = directives.processImage(html);
    html = directives.processModel(html);
    html = directives.processClient(html);
    html = directives.processScript(html);
    html = directives.processLoad(html);
    html = directives.processForm(html);
    html = directives.processField(html);
    html = directives.processInclude(html);
    
    // 5. Substitute mock data variables
    html = substituteVariables(html);
    
    // 6. Render components
    html = renderComponents(html);
    
    // 7. Template interpolation fallback
    html = html.replace(/\{\{\s*(\w+)\s*\}\}/g, '[$1]');
    html = html.replace(/\{\{\s*(\w+)\.(\w+)\s*\}\}/g, '[$1.$2]');
    
    return html;
}

/**
 * Substitute variables with mock data or demo values
 */
function substituteVariables(html) {
    // Substitute {{ variable.property }} or {{ variable.method() }}
    html = html.replace(/(\{\{|\{)\s*(\w+)\.(\w+)(?:\(\))?\s*(\}\}|\})/g, (match, open, obj, prop, close) => {
        if (mockData[obj]) {
            if (prop === 'len' && Array.isArray(mockData[obj])) {
                return mockData[obj].length;
            }
            if (mockData[obj][prop] !== undefined) {
                return mockData[obj][prop];
            }
        }
        // Check demo values
        const key = `${obj}.${prop}`;
        if (demoValues[key]) {
            return demoValues[key];
        }
        return match;
    });
    
    // Substitute simple {{ variable }} or { variable }
    html = html.replace(/(\{\{|\{)\s*(\w+)\s*(\}\}|\})/g, (match, open, varName, close) => {
        if (mockData[varName] !== undefined && typeof mockData[varName] !== 'object') {
            return mockData[varName];
        }
        return match;
    });
    
    return html;
}

/**
 * Render all components
 */
function renderComponents(html) {
    // StatCard
    html = html.replace(/<StatCard\s+([^>]*)\/>/g, (match, attrs) => 
        components.renderStatCard(attrs));
    
    // FeatureCard
    html = html.replace(/<FeatureCard\s+([^>]*)\/>/g, (match, attrs) => 
        components.renderFeatureCard(attrs));
    
    // Badge
    html = html.replace(/<Badge\s+([^>]*)>([\s\S]*?)<\/Badge>/g, (match, attrs, content) => 
        components.renderBadge(attrs, content));
    
    // Card
    html = html.replace(/<Card\s*([^>]*)>([\s\S]*?)<\/Card>/g, (match, attrs, content) => 
        components.renderCard(attrs, content));
    
    // Button
    html = html.replace(/<Button\s*([^>]*)>([\s\S]*?)<\/Button>/g, (match, attrs, content) => 
        components.renderButton(attrs, content));
    
    // TextInput
    html = html.replace(/<TextInput\s+([^>]*)\/>/g, (match, attrs) => 
        components.renderTextInput(attrs));
    
    // Select
    html = html.replace(/<Select\s+([^>]*)>([\s\S]*?)<\/Select>/g, (match, attrs, options) => 
        components.renderSelect(attrs, options));
    
    // Checkbox
    html = html.replace(/<Checkbox\s+([^>]*)\/>/g, (match, attrs) => 
        components.renderCheckbox(attrs));
    
    // FormGroup
    html = html.replace(/<FormGroup\s+([^>]*)>([\s\S]*?)<\/FormGroup>/g, (match, attrs, content) => 
        components.renderFormGroup(attrs, content));
    
    // NavItem
    html = html.replace(/<NavItem\s+([^>]*)>([\s\S]*?)<\/NavItem>/g, (match, attrs, content) => 
        components.renderNavItem(attrs, content));
    
    return html;
}

// Re-export for convenience
export { transpileNeutron } from './neutron.js';
export * as components from './components.js';
export * as directives from './directives.js';

// For backwards compatibility with non-module usage
if (typeof window !== 'undefined') {
    window.NucleusCompiler = {
        compile,
        transpileNeutron,
        components,
        directives
    };
}
