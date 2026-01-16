#![allow(dead_code)]
use nucleus_std::neutron::{create_effect, batch};
use serde::{Serialize, Deserialize};

// Domain Model
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub image: String,
}

#[derive(Clone, Debug)]
pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
}

// Global Store State
// Uses Signals for primary state and Computed for derived state
use nucleus_std::neutron::store;

#[store]
pub struct ShopStore {
    pub products: Vec<Product>,
    pub cart: Vec<CartItem>,
    pub cart_total: f64,
    pub cart_count: u32,
}

impl ShopStore {
    pub fn demo() -> Self {
        // Initialize with dummy products
        let products = vec![
            Product { id: 1, name: "Nucleus Pro".to_string(), price: 99.00, image: "cpu".to_string() },
            Product { id: 2, name: "Atom Reactor".to_string(), price: 299.00, image: "server".to_string() },
            Product { id: 3, name: "Neutron Star".to_string(), price: 499.00, image: "star".to_string() },
        ];

        let store = Self::new(products, Vec::new(), 0.0, 0);
        
        // Setup automatic derived state updates using effects
        // This effect will run whenever cart changes
        let cart = store.cart.clone();
        let total = store.cart_total.clone();
        let count = store.cart_count.clone();
        
        create_effect(move || {
            let items = cart.get();
            let new_total: f64 = items.iter().map(|i| i.product.price * i.quantity as f64).sum();
            let new_count: u32 = items.iter().map(|i| i.quantity).sum();
            
            // Batch updates so dependents only run once
            batch(|| {
                total.set(new_total);
                count.set(new_count);
            });
        }).forget(); // Effect lives for app lifetime
        
        store
    }

    pub fn add_to_cart(&self, product_id: u32) {
        let all_products = self.products.get();
        if let Some(product) = all_products.iter().find(|p| p.id == product_id) {
            self.cart.modify(|cart| {
                if let Some(item) = cart.iter_mut().find(|i| i.product.id == product_id) {
                    item.quantity += 1;
                } else {
                    cart.push(CartItem {
                        product: product.clone(),
                        quantity: 1,
                    });
                }
            });
            // No need to call recalculate - effect handles it automatically!
        }
    }
    
    pub fn remove_from_cart(&self, product_id: u32) {
        self.cart.modify(|cart| {
            cart.retain(|i| i.product.id != product_id);
        });
    }
    
    pub fn clear_cart(&self) {
        self.cart.set(Vec::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_initialization() {
        let store = ShopStore::demo();
        assert_eq!(store.products.get().len(), 3);
        assert_eq!(store.cart.get().len(), 0);
        assert_eq!(store.cart_total.get(), 0.0);
    }

    #[test]
    fn test_add_to_cart() {
        let store = ShopStore::demo();
        let product_id = 1; // 99.00
        
        store.add_to_cart(product_id);
        
        assert_eq!(store.cart.get().len(), 1);
        assert_eq!(store.cart_count.get(), 1);
        assert_eq!(store.cart_total.get(), 99.0);
        
        // Add same item again (quantity increase)
        store.add_to_cart(product_id);
        assert_eq!(store.cart.get().len(), 1);
        assert_eq!(store.cart_count.get(), 2);
        assert_eq!(store.cart_total.get(), 198.0);
    }

    #[test]
    fn test_add_multiple_types() {
        let store = ShopStore::demo();
        store.add_to_cart(1); // 99.0
        store.add_to_cart(2); // 299.0
        
        assert_eq!(store.cart.get().len(), 2);
        assert_eq!(store.cart_count.get(), 2);
        assert_eq!(store.cart_total.get(), 398.0);
    }
}
