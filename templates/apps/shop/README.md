# Nucleus Shop Example

A high-performance E-Commerce demonstration implementing the "Store Pattern" for state management.

## Features
- **Store Pattern**: Centralized `CartStore` managing application state.
- **Reactive UI**: Cart counter and total price update instantly via Signals.
- **Design**: Slide-over cart panel and responsive product grid.
- **No JS Framework**: Pure Vanilla JS + Nucleus reactivity.

## Tech Stack
- **Framework**: Nucleus V3
- **Styling**: Tailwind CSS
- **State**: Custom Pub/Sub Store (Client-side)
- **Data**: Static Product List (for demo)

## Running the Demo
```bash
# Run the server
nucleus run
```

Visit `http://localhost:3000`. Try adding items to the cart and watching the counter update instantly.
