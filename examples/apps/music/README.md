# Nucleus Music Example (Stream the Future)

A "Netflix-style" media streaming application demonstrating standard layout architecture and client-side interactivity.

![Nucleus Music Screenshot](/static/dashboard-preview.jpg)

## Features
- **Cinematic UI**: Full-screen hero video backgrounds and glassmorphism.
- **Horizontal Scrolling**: "Trending Now" rows implemented with Tailwind.
- **Islands Architecture**: Interactive "Like" buttons powered by WASM.
- **Optimized Assets**: Automatic WebP conversion for posters.

## Tech Stack
- **Framework**: Nucleus V3
- **Styling**: Tailwind CSS (CDN for demo simplicity)
- **State**: Neutron Signals (WASM)
- **Database**: Photon (SQLite) for media metadata

## Running the Demo
```bash
# 1. Install dependencies
nucleus install

# 2. Run the server
nucleus run
```

Visit `http://localhost:3000` to browse the catalog.
