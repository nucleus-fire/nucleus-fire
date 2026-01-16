# Static Export Guide

Nucleus supports static site generation (SSG) for deploying to CDNs and static hosting platforms.

## Overview

The `nucleus export` command converts your Nucleus app into static HTML files, ready for deployment to:
- Netlify
- Vercel
- Cloudflare Pages
- GitHub Pages
- Any static hosting

## Quick Start

```bash
# Basic export
nucleus export

# With options
nucleus export --output dist --base-url https://example.com

# Interactive wizard
nucleus export --wizard
```

## Export Configuration

### Command Options

| Option | Default | Description |
|--------|---------|-------------|
| `--output` | `dist` | Output directory |
| `--base-url` | - | Base URL for links |
| `--incremental` | false | Only rebuild changed files |
| `--wizard` | false | Run interactive wizard |
| `--platform` | - | Target platform (netlify, vercel, etc.) |

### Example Commands

```bash
# Export for Netlify
nucleus export --platform netlify

# Incremental build
nucleus export --incremental

# Custom output directory
nucleus export --output public
```

## What Gets Exported

The export process handles:

1. **HTML Pre-rendering** - All routes converted to static HTML
2. **Static Assets** - CSS, JS, images copied to output
3. **Image Optimization** - PNG/JPG converted to WebP
4. **SEO Files** - sitemap.xml, robots.txt auto-generated
5. **Platform Configs** - _redirects, vercel.json, etc.

## Directory Structure

After export, your `dist` folder looks like:

```
dist/
├── index.html          # Home page
├── docs/
│   └── index.html      # /docs route
├── assets/
│   ├── output.css
│   ├── home.js
│   └── images/
│       └── hero.webp   # Optimized images
├── sitemap.xml
├── robots.txt
├── 404.html
└── _redirects          # Platform-specific
```

## Platform-Specific Deployment

### Netlify

```bash
nucleus export --platform netlify
```

Generates:
- `_redirects` for SPA fallback
- `_headers` for cache control

Deploy:
```bash
nucleus publish --platform netlify
```

### Vercel

```bash
nucleus export --platform vercel
```

Generates:
- `vercel.json` with routing config

Deploy:
```bash
nucleus publish --platform vercel
```

### GitHub Pages

```bash
nucleus export --platform github
```

Generates:
- `.nojekyll` to disable Jekyll

Deploy via GitHub Actions:
```yaml
- name: Deploy to Pages
  uses: peaceiris/actions-gh-pages@v3
  with:
    github_token: ${{ secrets.GITHUB_TOKEN }}
    publish_dir: ./dist
```

### Cloudflare Pages

```bash
nucleus export --platform cloudflare
```

Generates:
- `_routes.json` for routing

## Incremental Builds

For large sites, incremental builds only rebuild changed files:

```bash
nucleus export --incremental
```

The cache is stored in `.nucleus/export-cache.json`.

## Image Optimization

Images are automatically optimized during export:

- **WebP Conversion** - PNG/JPG → WebP (smaller files)
- **Original Preserved** - Original files kept as fallback
- **Lazy Loading** - `loading="lazy"` added to images

To disable:
```bash
python -c "# Set convert_webp: false in export config"
```

## Dynamic Routes

Dynamic routes (`/users/[id]`) cannot be statically generated without data. Options:

1. **Pre-generate with data** - Provide known IDs
2. **Client-side fallback** - Fetch data on client
3. **Hybrid** - Use SSG for static pages, SSR for dynamic

## Environment Variables

Export respects environment variables:

```bash
BASE_URL=https://mysite.com nucleus export
```

Or in `nucleus.config`:
```toml
[export]
base_url = "https://mysite.com"
```

## Troubleshooting

### Empty pages
- Ensure views have `<n:view>` wrapper
- Check for JavaScript errors in browser console

### Missing assets
- Verify files exist in `static/` directory
- Check asset paths are relative (start with `/`)

### Build errors
- Run `nucleus run` first to catch NCL syntax errors
- Check for missing dependencies

## Best Practices

1. **Clean builds** - Delete `dist/` before production export
2. **Check links** - Verify all internal links work
3. **Test locally** - `cd dist && python3 -m http.server`
4. **CI/CD** - Automate exports in your build pipeline

## Related Guides

- [Deployment Guide](#23_deployment_guide) - Server deployment
- [Performance](#13_performance_benchmarks) - Optimization tips
