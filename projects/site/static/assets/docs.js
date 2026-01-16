/**
 * Nucleus Documentation - Client-side JavaScript
 * Renders markdown documentation with marked.js
 */

let docsManifest = [];
let currentDoc = null;
let collapsedSections = JSON.parse(localStorage.getItem('docs-collapsed') || '{}');

const categories = {
    'Getting Started': [
        'README',
        '01_getting_started',
        '24_quick_start_tutorial',
        '25_project_structure',
        '02_core_concepts'
    ],
    'Guides': [
        '20_database_guide',
        '21_authentication_guide',
        '27_social_login_guide',
        '22_api_development',
        '07_forms_and_validation',
        '26_components_guide',
        '52_client_navigation_guide',
        '30_websocket_guide',
        '32_middleware_guide',
        '33_error_handling_guide',
        '25_federation_guide',
        '25_email_guide',
        '26_image_processing_guide',
        '27_offline_sync_guide',
        '28_i18n_guide',
        '29_analytics_guide',
        '23_deployment_guide',
        '31_static_export_guide',
        '26_testing_guide',
        '40_rate_limit_headers_guide',
        '41_pool_monitor_guide',
        '51_developer_tools_guide',
        '49_payments_guide',
        '50_crypto_chain_guide'
    ],
    'Reference': [
        '19_syntax_reference',
        '04_stdlib_reference',
        '17_cli_reference',
        '03_compiler_reference',
        '15_state_management',
        '18_complete_reference',
        'configuration'
    ],
    'Examples & Recipes': [
        '16_examples',
        '06_building_a_cms'
    ],
    'Advanced': [
        '13_performance_benchmarks',
        '14_dependency_management',
        '13_architecture_improvements',
        'web_standards',
        'type_safety',
        'tooling'
    ]
};

async function loadDocsManifest() {
    try {
        const res = await fetch('/docs/manifest.json');
        docsManifest = await res.json();
        buildNavigation();

        const hash = window.location.hash.slice(1);
        if (hash) {
            loadDoc(hash);
        }
    } catch (e) {
        const nav = document.getElementById('docs-nav');
        if (nav) {
            nav.innerHTML = '<p class="error-msg">Failed to load documentation manifest.</p>';
        }
    }
}

function buildNavigation() {
    const nav = document.getElementById('docs-nav');
    if (!nav) return;
    nav.innerHTML = '';

    for (const [category, slugs] of Object.entries(categories)) {
        const docs = docsManifest.filter(d => slugs.includes(d.slug));
        if (docs.length === 0) continue;

        const section = createNavSection(category, docs);
        nav.appendChild(section);
    }

    const categorized = Object.values(categories).flat();
    const other = docsManifest.filter(d =>
        !categorized.includes(d.slug) &&
        !d.slug.startsWith('00') &&
        d.slug !== 'README'
    );

    if (other.length > 0) {
        const section = createNavSection('Other', other);
        nav.appendChild(section);
    }
}

function createNavSection(category, docs) {
    const isCollapsed = collapsedSections[category] === true;
    const section = document.createElement('div');
    section.className = 'docs-nav-section' + (isCollapsed ? ' collapsed' : '');
    section.dataset.category = category;

    const header = document.createElement('div');
    header.className = 'docs-nav-header';
    header.innerHTML = `
        <h4 class="docs-nav-title">${category}</h4>
        <svg class="docs-nav-toggle" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M4.427 5.427a.75.75 0 0 1 1.06 0L8 7.94l2.513-2.513a.75.75 0 1 1 1.06 1.06l-3.043 3.043a.75.75 0 0 1-1.06 0L4.427 6.487a.75.75 0 0 1 0-1.06z"/>
        </svg>
    `;
    header.addEventListener('click', () => toggleSection(category, section));
    section.appendChild(header);

    const list = document.createElement('ul');
    list.className = 'docs-nav-list';

    docs.forEach(doc => {
        const li = document.createElement('li');
        const link = document.createElement('a');
        link.href = '#' + doc.slug;
        link.className = 'docs-nav-link';
        link.dataset.slug = doc.slug;
        link.dataset.title = doc.title.toLowerCase();
        link.textContent = doc.title;
        link.addEventListener('click', function(e) {
            e.preventDefault();
            loadDoc(doc.slug);
        });
        li.appendChild(link);
        list.appendChild(li);
    });

    section.appendChild(list);
    return section;
}

function toggleSection(category, section) {
    section.classList.toggle('collapsed');
    const isCollapsed = section.classList.contains('collapsed');
    collapsedSections[category] = isCollapsed;
    localStorage.setItem('docs-collapsed', JSON.stringify(collapsedSections));
}

async function loadDoc(slug) {
    const doc = docsManifest.find(d => d.slug === slug);
    if (!doc) return;

    currentDoc = slug;
    window.location.hash = slug;

    document.querySelectorAll('.docs-nav-link').forEach(link => {
        link.classList.toggle('active', link.getAttribute('href') === '#' + slug);
    });

    const activeLink = document.querySelector(`.docs-nav-link[href="#${slug}"]`);
    if (activeLink) {
        const section = activeLink.closest('.docs-nav-section');
        if (section && section.classList.contains('collapsed')) {
            section.classList.remove('collapsed');
            const cat = section.dataset.category;
            collapsedSections[cat] = false;
            localStorage.setItem('docs-collapsed', JSON.stringify(collapsedSections));
        }
    }

    const landing = document.getElementById('docs-landing');
    if (landing) landing.style.display = 'none';

    try {
        const res = await fetch('/docs/raw/' + doc.file);
        if (!res.ok) throw new Error(`Fetch error: ${res.status}`);
        const markdown = await res.text();

        const html = marked.parse(markdown);
        const bodyEl = document.getElementById('docs-body');
        bodyEl.innerHTML = `
            <article class="docs-article fade-in">
                ${html}
            </article>
        `;

        document.querySelector('.docs-content').scrollTop = 0;
        window.scrollTo(0, 0);

        if (window.hljs) {
            hljs.highlightAll();
        }

        addHeadingIds();
        generateTOC();
        generateBreadcrumbs(doc);
        generateArticleNav(slug);
        addCopyButtons();
        handleContentLinks();
        setupScrollSpy();
    } catch (e) {
        document.getElementById('docs-body').innerHTML =
            '<p class="error-msg">Failed to load document: ' + e.message + '</p>';
    }
}

function generateTOC() {
    const tocEl = document.getElementById('docs-toc');
    if (!tocEl) return;

    tocEl.innerHTML = '';
    const headings = document.querySelectorAll('.docs-article h2, .docs-article h3');

    if (headings.length === 0) {
        tocEl.innerHTML = '<p class="text-sm text-gray-500 italic">No subsections</p>';
        return;
    }

    const ul = document.createElement('ul');
    ul.className = 'toc-list';

    headings.forEach(heading => {
        if (!heading.id) return;

        const li = document.createElement('li');
        const a = document.createElement('a');
        a.href = '#' + heading.id;
        a.textContent = heading.textContent;
        a.className = heading.tagName === 'H2' ? 'toc-link-h2' : 'toc-link-h3';

        a.addEventListener('click', (e) => {
            e.preventDefault();
            heading.scrollIntoView({ behavior: 'smooth', block: 'start' });
        });

        li.appendChild(a);
        ul.appendChild(li);
    });

    tocEl.appendChild(ul);
}

function generateBreadcrumbs(currentDocObj) {
    const breadEl = document.getElementById('docs-breadcrumbs');
    if (!breadEl) return;

    let category = 'Docs';
    for (const [cat, slugs] of Object.entries(categories)) {
        if (slugs.includes(currentDocObj.slug)) {
            category = cat;
            break;
        }
    }

    breadEl.innerHTML = `
        <a href="/docs" onclick="location.reload(); return false;">Docs</a>
        <span class="separator">/</span>
        <span class="category">${category}</span>
    `;
}

function generateArticleNav(currentSlug) {
    const navEl = document.getElementById('docs-article-nav');
    if (!navEl) return;
    navEl.innerHTML = '';

    const allSlugs = [];
    Object.values(categories).forEach(slugs => allSlugs.push(...slugs));

    const idx = allSlugs.indexOf(currentSlug);
    if (idx === -1) return;

    const prevSlug = idx > 0 ? allSlugs[idx - 1] : null;
    const nextSlug = idx < allSlugs.length - 1 ? allSlugs[idx + 1] : null;

    let html = '<div class="article-nav-container">';

    if (prevSlug) {
        const prevDoc = docsManifest.find(d => d.slug === prevSlug);
        if (prevDoc) {
            html += `
                <a href="#${prevSlug}" class="article-nav-prev" onclick="loadDoc('${prevSlug}'); return false;">
                    <span class="label">← Previous</span>
                    <span class="title">${prevDoc.title}</span>
                </a>
            `;
        } else {
            html += '<div></div>';
        }
    } else {
        html += '<div></div>';
    }

    if (nextSlug) {
        const nextDoc = docsManifest.find(d => d.slug === nextSlug);
        if (nextDoc) {
            html += `
                <a href="#${nextSlug}" class="article-nav-next" onclick="loadDoc('${nextSlug}'); return false;">
                    <span class="label">Next →</span>
                    <span class="title">${nextDoc.title}</span>
                </a>
            `;
        }
    }

    html += '</div>';
    navEl.innerHTML = html;
}

function setupScrollSpy() {
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const id = entry.target.id;
                document.querySelectorAll('.toc-list a').forEach(link => {
                    link.classList.toggle('active', link.getAttribute('href') === '#' + id);
                });
            }
        });
    }, { rootMargin: '-100px 0px -66%' });

    document.querySelectorAll('.docs-article h2, .docs-article h3').forEach(h => {
        observer.observe(h);
    });
}

function addHeadingIds() {
    document.querySelectorAll('.docs-article h2, .docs-article h3').forEach(heading => {
        if (heading.id) return;
        const text = heading.textContent.trim();
        const id = text.toLowerCase()
            .replace(/[^\w\s-]/g, '')
            .replace(/\s+/g, '-')
            .replace(/-+/g, '-');
        heading.id = id;
    });
}

function addCopyButtons() {
    document.querySelectorAll('pre code').forEach(block => {
        if (block.parentElement.querySelector('.copy-btn')) return;

        const wrapper = block.parentElement;
        const btn = document.createElement('button');
        btn.className = 'copy-btn';
        btn.textContent = 'Copy';
        btn.addEventListener('click', async function() {
            try {
                await copyText(block.textContent);
                btn.textContent = 'Copied!';
            } catch {
                btn.textContent = 'Failed';
            }
            setTimeout(function() { btn.textContent = 'Copy'; }, 2000);
        });
        wrapper.appendChild(btn);
    });
}

async function copyText(text) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
        return navigator.clipboard.writeText(text);
    }

    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.left = '-9999px';
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();

    return new Promise((resolve, reject) => {
        try {
            const successful = document.execCommand('copy');
            document.body.removeChild(textArea);
            if (successful) {
                resolve();
            } else {
                reject(new Error('Copy command failed'));
            }
        } catch (err) {
            document.body.removeChild(textArea);
            reject(err);
        }
    });
}

function handleContentLinks() {
    document.querySelectorAll('.docs-article a[href^="#"]').forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const href = link.getAttribute('href').substring(1);

            if (docsManifest.find(d => d.slug === href)) {
                loadDoc(href);
                return;
            }

            const target = document.getElementById(href);
            if (target) {
                target.scrollIntoView({ behavior: 'smooth', block: 'start' });
            }
        });
    });
}

function filterDocs() {
    const searchInput = document.getElementById('doc-search');
    if (!searchInput) return;

    const query = searchInput.value.toLowerCase().trim();
    const links = document.querySelectorAll('.docs-nav-link');
    const sections = document.querySelectorAll('.docs-nav-section');

    if (query === '') {
        links.forEach(link => link.parentElement.style.display = 'block');
        sections.forEach(section => {
            section.style.display = 'block';
            const cat = section.dataset.category;
            if (collapsedSections[cat]) {
                section.classList.add('collapsed');
            } else {
                section.classList.remove('collapsed');
            }
        });
        return;
    }

    links.forEach(link => {
        const text = (link.textContent + ' ' + (link.dataset.slug || '')).toLowerCase();
        link.parentElement.style.display = text.includes(query) ? 'block' : 'none';
    });

    sections.forEach(section => {
        const visibleLinks = Array.from(section.querySelectorAll('.docs-nav-link'))
            .filter(l => l.parentElement.style.display !== 'none');
        section.style.display = visibleLinks.length > 0 ? 'block' : 'none';

        if (visibleLinks.length > 0) {
            section.classList.remove('collapsed');
        }
    });
}

function initDocs() {
    if (!document.getElementById('docs-nav')) return;
    loadDocsManifest();

    const searchInput = document.getElementById('doc-search');
    if (searchInput) {
        let timeout;
        searchInput.addEventListener('keyup', () => {
            clearTimeout(timeout);
            timeout = setTimeout(filterDocs, 150);
        });
    }
}

document.addEventListener('DOMContentLoaded', initDocs);
window.addEventListener('nucleus:navigate', initDocs);
window.addEventListener('hashchange', function() {
    const hash = window.location.hash.slice(1);
    if (hash && hash !== currentDoc && docsManifest.find(d => d.slug === hash)) {
        loadDoc(hash);
    }
});
