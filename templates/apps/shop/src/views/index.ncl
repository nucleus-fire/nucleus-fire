<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nucleus Shop</title>
    <link href="/assets/styles.css" rel="stylesheet">
    <script>
        // State Management Best Practice: The Store Pattern
        // In a real Nucleus app, this logic lives in Rust (WASM).
        // For this demo, we simulate the Rust Store API in JS.
        
        class CartStore {
            constructor() {
                // Primary State
                this.products = [
                    { id: 1, name: "Nucleus Pro", price: 99.00, desc: "High-performance framework license" },
                    { id: 2, name: "Atom Reactor", price: 299.00, desc: "Dedicated runtime server" },
                    { id: 3, name: "Neutron Star", price: 499.00, desc: "Enterprise state management" },
                    { id: 4, name: "Cloud Orbit", price: 19.00, desc: "Monthly hosting subscription" },
                ];
                this.cart = [];
                
                // Reactive Subscribers
                this.listeners = [];
            }
            
            // Action: Add to Cart
            add(productId) {
                const product = this.products.find(p => p.id === productId);
                const item = this.cart.find(i => i.id === productId);
                
                if (item) {
                    item.quantity++;
                } else {
                    this.cart.push({ ...product, quantity: 1 });
                }
                this.notify();
            }
            
            // Action: Remove from Cart
            remove(productId) {
                this.cart = this.cart.filter(i => i.id !== productId);
                this.notify();
            }
            
            // Derived State: Total Price
            get total() {
                return this.cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
            }
            
            // Derived State: Item Count
            get count() {
                return this.cart.reduce((sum, item) => sum + item.quantity, 0);
            }
            
            subscribe(fn) {
                this.listeners.push(fn);
                fn(this); // Initial call
            }
            
            notify() {
                this.listeners.forEach(fn => fn(this));
            }
        }
        
        const store = new CartStore();
    </script>
</head>
<body class="bg-gray-50 font-sans text-gray-900">

    <!-- Navbar -->
    <nav class="bg-white border-b border-gray-200 sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16">
                <div class="flex items-center">
                    <span class="font-bold text-2xl tracking-tight text-indigo-600">Nucleus<span class="text-gray-900">Store</span></span>
                </div>
                <div class="flex items-center">
                    <button onclick="toggleCart()" class="relative p-2 text-gray-400 hover:text-gray-500">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path></svg>
                        <span id="nav-count" class="absolute top-0 right-0 inline-flex items-center justify-center px-2 py-1 text-xs font-bold leading-none text-white transform translate-x-1/4 -translate-y-1/4 bg-red-600 rounded-full hidden">0</span>
                    </button>
                </div>
            </div>
        </div>
    </nav>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div class="lg:grid lg:grid-cols-12 lg:gap-x-12 lg:items-start">
            
            <!-- Product Grid -->
            <section class="lg:col-span-12">
                <h2 class="text-2xl font-extrabold tracking-tight text-gray-900 mb-6">Featured Products</h2>
                
                <div class="grid grid-cols-1 gap-y-10 sm:grid-cols-2 gap-x-6 lg:grid-cols-3 xl:gap-x-8" id="product-grid">
                    <!-- Products injected by JS -->
                </div>
            </section>
        </div>
    </div>

    <!-- Cart Slide-over -->
    <div id="cart-panel" class="fixed inset-0 overflow-hidden z-50 pointer-events-none">
        <div class="absolute inset-0 overflow-hidden">
            <div class="absolute inset-0 bg-gray-500 bg-opacity-75 transition-opacity opacity-0 pointer-events-none" id="cart-backdrop" onclick="toggleCart()"></div>
            <div class="fixed inset-y-0 right-0 pl-10 max-w-full flex transform transition-transform translate-x-full" id="cart-container">
                <div class="w-screen max-w-md pointer-events-auto">
                    <div class="h-full flex flex-col bg-white shadow-xl overflow-y-scroll">
                        <div class="flex-1 py-6 overflow-y-auto px-4 sm:px-6">
                            <div class="flex items-start justify-between">
                                <h2 class="text-lg font-medium text-gray-900">Shopping Cart</h2>
                                <button onclick="toggleCart()" class="text-gray-400 hover:text-gray-500">
                                    <span class="sr-only">Close panel</span>
                                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                                </button>
                            </div>

                            <div class="mt-8">
                                <div class="flow-root">
                                    <ul role="list" class="-my-6 divide-y divide-gray-200" id="cart-items">
                                        <!-- Items injected -->
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div class="border-t border-gray-200 py-6 px-4 sm:px-6">
                            <div class="flex justify-between text-base font-medium text-gray-900">
                                <p>Subtotal</p>
                                <p id="cart-total">$0.00</p>
                            </div>
                            <p class="mt-0.5 text-sm text-gray-500">Shipping and taxes calculated at checkout.</p>
                            <div class="mt-6">
                                <a href="#" class="flex justify-center items-center px-6 py-3 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700">Checkout</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script>
        // --- View Logic ---
        
        // 1. Render Products
        const grid = document.getElementById('product-grid');
        store.products.forEach(p => {
            const el = document.createElement('div');
            el.className = 'group relative bg-white border border-gray-200 rounded-xl p-6 hover:shadow-lg transition';
            el.innerHTML = `
                <div class="aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-md bg-gray-200 lg:aspect-none lg:h-40 mb-4 flex items-center justify-center text-4xl text-gray-400">
                   📦
                </div>
                <div class="mt-4 flex justify-between">
                    <div>
                        <h3 class="text-sm text-gray-700"><a href="#"><span aria-hidden="true" class="absolute inset-0"></span>${p.name}</a></h3>
                        <p class="mt-1 text-sm text-gray-500">${p.desc}</p>
                    </div>
                    <p class="text-sm font-medium text-gray-900">$${p.price}</p>
                </div>
                <button onclick="event.stopPropagation(); addToCart(${p.id})" class="mt-4 w-full bg-gray-100 text-gray-800 py-2 rounded-lg text-sm font-medium hover:bg-indigo-600 hover:text-white transition z-10 relative">Add to Bag</button>
            `;
            grid.appendChild(el);
        });

        // 2. Subscribe to Store Updates
        store.subscribe(state => {
            // Update Nav Count
            const badge = document.getElementById('nav-count');
            if (state.count > 0) {
                badge.innerText = state.count;
                badge.classList.remove('hidden');
            } else {
                badge.classList.add('hidden');
            }
            
            // Update Total
            document.getElementById('cart-total').innerText = '$' + state.total.toFixed(2);
            
            // Update Cart Items
            const list = document.getElementById('cart-items');
            list.innerHTML = '';
            if (state.cart.length === 0) {
                list.innerHTML = '<p class="text-center text-gray-500 py-8">Your cart is empty.</p>';
                return;
            }
            
            state.cart.forEach(item => {
                const li = document.createElement('li');
                li.className = 'py-6 flex';
                li.innerHTML = `
                    <div class="h-24 w-24 flex-shrink-0 overflow-hidden rounded-md border border-gray-200 bg-gray-100 flex items-center justify-center text-2xl">📦</div>
                    <div class="ml-4 flex flex-1 flex-col">
                        <div>
                            <div class="flex justify-between text-base font-medium text-gray-900">
                                <h3><a href="#">${item.name}</a></h3>
                                <p class="ml-4">$${(item.price * item.quantity).toFixed(2)}</p>
                            </div>
                            <p class="mt-1 text-sm text-gray-500">${item.desc}</p>
                        </div>
                        <div class="flex flex-1 items-end justify-between text-sm">
                            <p class="text-gray-500">Qty ${item.quantity}</p>
                            <div class="flex">
                                <button type="button" onclick="store.remove(${item.id})" class="font-medium text-indigo-600 hover:text-indigo-500">Remove</button>
                            </div>
                        </div>
                    </div>
                `;
                list.appendChild(li);
            });
        });

        // 3. User Actions
        function addToCart(id) {
            store.add(id);
            // Open cart if closed
            const panel = document.getElementById('cart-container');
            if(!panel.classList.contains('translate-x-0')) toggleCart();
        }
        
        function toggleCart() {
            const backdrop = document.getElementById('cart-backdrop');
            const container = document.getElementById('cart-container');
            const root = document.getElementById('cart-panel');
            
            if (container.classList.contains('translate-x-full')) {
                // Open
                root.classList.remove('pointer-events-none');
                backdrop.classList.remove('opacity-0', 'pointer-events-none');
                container.classList.remove('translate-x-full');
                container.classList.add('translate-x-0');
            } else {
                // Close
                root.classList.add('pointer-events-none');
                backdrop.classList.add('opacity-0', 'pointer-events-none');
                container.classList.add('translate-x-full');
                container.classList.remove('translate-x-0');
            }
        }
    </script>
</body>
</html>
