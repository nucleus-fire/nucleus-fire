/**
 * Nucleus Neutron Transpiler
 * Converts Neutron/Rust-like Island syntax to executable JavaScript
 * 
 * Handles:
 * - Signal::new() declarations
 * - computed() expressions
 * - {variable} bindings
 * - onclick={signal.update(...)} handlers
 * 
 * @module playground/compiler/neutron
 */

/**
 * Transpile Neutron source code to HTML + runtime script
 * 
 * @param {string} source - Neutron/Rust-like source code
 * @returns {{ html: string, script: string }} - Transpiled HTML and runtime script
 */
export function transpileNeutron(source) {
    // 1. Extract State (Signals)
    // let count = Signal::new(0);
    const signals = {};
    let scriptInit = '';
    
    // Regex for Signal::new
    const signalRegex = /let\s+(\w+)\s*=\s*Signal::new\(([^)]+)\);/g;
    let match;
    while ((match = signalRegex.exec(source)) !== null) {
        const [_, name, val] = match;
        signals[name] = { type: 'signal', initial: val.trim() };
        scriptInit += `state.${name} = ${val.trim()};\n`;
    }

    // 2. Extract Computed
    // let double = computed(count.clone(), |c| c * 2);
    const computedRegex = /let\s+(\w+)\s*=\s*computed\s*\(\s*(\w+)\.clone\(\)\s*,\s*\|[^|]+\|\s*([^)]+)\);/g;
    while ((match = computedRegex.exec(source)) !== null) {
        const [_, name, dep, expr] = match;
        signals[name] = { type: 'computed', dep, expr: expr.replace(/[a-z]\s/g, `state.${dep} `) }; 
    }

    // 3. Process Template & Bindings
    // Remove rust-specific imports/setup lines (everything before first proper tag)
    const tagMatch = source.match(/<[a-zA-Z]/);
    const firstTagIndex = tagMatch ? tagMatch.index : -1;
    let html = firstTagIndex >= 0 ? source.substring(firstTagIndex) : '';

    // Replace {variable} -> <span data-n-bind="variable"></span>
    html = html.replace(/\{(\w+)\}/g, (match, varName) => {
        return `<span data-n-bind="${varName}"></span>`;
    });

    // Replace onclick closures with multi-line support
    // Handles both syntaxes:
    //   onclick={count.update(|c| *c += 1)}
    //   onclick={|_| count.update(|c| *c += 1)}
    // Uses [\s\S] instead of . to match across newlines
    html = html.replace(/onclick\s*=\s*\{(?:\|_\|\s*)?[\s\S]*?(\w+)\.update[\s\S]*?(\+=|-=|=)\s*(\d+)[\s\S]*?\}/gi, 
        (m, sig, op, val) => `onclick="return false" data-n-action="${sig}:${op}:${val}"`);

    // Generate Runtime Script
    const script = `
    (function(elRoot) {
        const state = {};
        ${scriptInit}

        function update() {
            // Update bindings
            elRoot.querySelectorAll('[data-n-bind]').forEach(el => {
                const key = el.dataset.nBind;
                if (state[key] !== undefined) el.textContent = state[key];
                
                // Handle computed mocks (very naive)
                if (key === 'double' && state.count !== undefined) el.textContent = state.count * 2; 
            });
            
            // Dynamic class for count
            const countEl = elRoot.querySelector('[data-n-bind="count"]');
            if(countEl && state.count !== undefined) {
                 countEl.parentElement.className = 'text-5xl font-bold mb-4 transition-all transform ' + (state.count % 2 === 0 ? 'text-indigo-600' : 'text-violet-600');
            }
        }

        // Attach listeners
        elRoot.querySelectorAll('[data-n-action]').forEach(btn => {
            if(btn.dataset.nAttached) return;
            btn.dataset.nAttached = 'true';
            btn.addEventListener('click', (e) => {
                e.preventDefault();
                const [sig, op, val] = btn.dataset.nAction.split(':');
                const numVal = parseInt(val, 10);
                
                if (state[sig] !== undefined) {
                    if (op === '+=') state[sig] += numVal;
                    if (op === '-=') state[sig] -= numVal;
                    if (op === '=') state[sig] = numVal;
                    update();
                }
            });
        });

        // Initial render
        update();
    })(document.currentScript.parentElement);
    `;

    return { html, script };
}

// For backwards compatibility with non-module usage
if (typeof window !== 'undefined') {
    window.transpileNeutron = transpileNeutron;
}
