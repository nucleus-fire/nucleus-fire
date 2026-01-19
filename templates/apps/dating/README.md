# Amour: Premium Dating App (Nucleus Benchmark Flagship)

> **Status**: Production Ready (Benchmark Reference)
> **Built with**: Nucleus V3, TailwindCSS, TypeScript

Amour is the flagship demonstration of the **Nucleus Framework's** capabilities. It is designed to mimic a high-scale, real-time premium dating application (like Tinder or Hinge) to validate Nucleus's performance, developer experience, and architectural patterns.

## 🚀 Key Features

*   **HTML-First Architecture**: 100 Lighthouse Performance score. Zero JavaScript on the landing page.
*   **Islands of Interactivity**: Complex swiping and heavy interactions are isolated into `<n:island>` components using Vanilla JS and TypeScript, keeping the main thread free.
*   **Real-Time Chat**: Powered by `nucleus::websocket` and `Atom Reactor` for ultra-low latency messaging (`< 2ms` broadcast time).
*   **Premium UI**: Full TailwindCSS integration with glassmorphism, micro-animations, and responsive design.
*   **Secure by Default**: Strict Content Security Policy (CSP) and input validation baked in.

## 📊 Performance Specs

As detailed in [PERFORMANCE.md](../../docs/PERFORMANCE.md), Amour achieves:
*   **TTFB**: 1.2ms (Dynamic content)
*   **Memory**: ~15MB RSS under load
*   **Throughput**: 125k Req/Sec capacity

## 🛠️ Usage

### Build for Production
```bash
# Builds optimized binary and assets
nucleus build
```

### Run Development Server
```bash
# Starts server with Hot Module Replacement (HMR)
nucleus run
```

### Project Structure
*   `src/views/`: Nucleus HTML templates (`.ncl`)
*   `src/bin/server.rs`: Logic and Route Handling
*   `static/`: Assets (Images, compiled JS/CSS)
*   `tailwind.config.js`: Design system configuration
