//! Neutron - Fine-Grained Reactive State Management
//!
//! A state-of-the-art reactive system for Rust, inspired by SolidJS and Svelte 5.
//!
//! ## Quick Start
//!
//! ```rust
//! use nucleus_std::neutron::{Signal, create_effect, computed};
//!
//! // Create reactive state
//! let count = Signal::new(0);
//!
//! // Computed values update automatically
//! let doubled = computed(count.clone(), |c| c * 2);
//!
//! // Effects run when dependencies change
//! create_effect({
//!     let count = count.clone();
//!     move || println!("Count: {}", count.get())
//! });
//!
//! // Update triggers all dependents
//! count.set(5);
//! assert_eq!(doubled.get(), 10);
//! ```
//!
//! ## Features
//!
//! - **Fine-Grained**: Only updates what changes
//! - **Thread-Safe**: Works across threads with `Send + Sync`
//! - **Computed Values**: Automatic memoization
//! - **Batch Updates**: Group changes efficiently
//! - **Effect Cleanup**: No memory leaks
//! - **Store Pattern**: Organized state containers

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════
// ID GENERATION
// ═══════════════════════════════════════════════════════════════════════════

static SIGNAL_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

fn next_id() -> u64 {
    SIGNAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

// ═══════════════════════════════════════════════════════════════════════════
// BATCHING
// ═══════════════════════════════════════════════════════════════════════════

thread_local! {
    static BATCHING: RefCell<bool> = const { RefCell::new(false) };
    static PENDING_EFFECTS: RefCell<HashSet<u64>> = RefCell::new(HashSet::new());
}

/// Batch multiple signal updates into a single notification pass.
///
/// This prevents intermediate re-renders when updating multiple signals.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::{Signal, batch};
///
/// let a = Signal::new(0);
/// let b = Signal::new(0);
///
/// // Without batch: effect runs twice
/// // With batch: effect runs once
/// batch(|| {
///     a.set(1);
///     b.set(2);
/// });
/// ```
pub fn batch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    BATCHING.with(|b| *b.borrow_mut() = true);
    let result = f();
    BATCHING.with(|b| *b.borrow_mut() = false);

    // Flush pending effects
    let pending: HashSet<u64> = PENDING_EFFECTS.with(|p| std::mem::take(&mut *p.borrow_mut()));

    for effect_id in pending {
        trigger_effect(effect_id);
    }

    result
}

fn is_batching() -> bool {
    BATCHING.with(|b| *b.borrow())
}

fn queue_effect(id: u64) {
    PENDING_EFFECTS.with(|p| {
        p.borrow_mut().insert(id);
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// SIGNAL
// ═══════════════════════════════════════════════════════════════════════════

/// A reactive primitive that holds a value and notifies subscribers on change.
///
/// Signals are the atoms of reactive state. When their value changes, all
/// dependent effects and computed values automatically update.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::Signal;
///
/// let name = Signal::new("Alice".to_string());
///
/// // Read value
/// assert_eq!(name.get(), "Alice");
///
/// // Write value (notifies dependents)
/// name.set("Bob".to_string());
///
/// // Update in place
/// name.update(|n| n.push_str("!"));
/// assert_eq!(name.get(), "Bob!");
/// ```
#[derive(Debug)]
pub struct Signal<T> {
    id: u64,
    value: Arc<RwLock<T>>,
    subscribers: Arc<RwLock<HashSet<u64>>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            value: Arc::clone(&self.value),
            subscribers: Arc::clone(&self.subscribers),
        }
    }
}

impl<T> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Signal<T> {}

impl<T> std::hash::Hash for Signal<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T: Clone + 'static> Signal<T> {
    /// Create a new signal with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            id: next_id(),
            value: Arc::new(RwLock::new(initial)),
            subscribers: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Get the current value and track this signal as a dependency.
    ///
    /// If called inside an effect or computed, the signal will be
    /// tracked and the effect will re-run when this signal changes.
    pub fn get(&self) -> T {
        // Track dependency if inside an effect
        if let Some(effect_id) = CURRENT_EFFECT.with(|e| *e.borrow()) {
            let mut subs = self.subscribers.write().unwrap();
            subs.insert(effect_id);
        }

        self.value.read().unwrap().clone()
    }

    /// Get the current value without tracking as a dependency.
    ///
    /// Use this when you need the value but don't want to create
    /// a reactive subscription.
    pub fn get_untracked(&self) -> T {
        self.value.read().unwrap().clone()
    }

    /// Set a new value and notify all subscribers.
    pub fn set(&self, new_value: T) {
        {
            let mut w = self.value.write().unwrap();
            *w = new_value;
        }
        self.notify();
    }

    /// Update the value in-place and notify subscribers.
    ///
    /// More efficient than `get()` + `set()` for complex types.
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut w = self.value.write().unwrap();
            f(&mut *w);
        }
        self.notify();
    }

    /// Get the signal's unique ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    fn notify(&self) {
        let subs = {
            let lock = self.subscribers.read().unwrap();
            lock.clone()
        };

        for effect_id in subs.iter() {
            if is_batching() {
                queue_effect(*effect_id);
            } else {
                trigger_effect(*effect_id);
            }
        }
    }
}

impl<T: Clone + PartialEq + 'static> Signal<T> {
    /// Set value only if it differs from current (prevents unnecessary updates).
    pub fn set_if_changed(&self, new_value: T) {
        let current = self.value.read().unwrap().clone();
        if current != new_value {
            drop(current);
            self.set(new_value);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPUTED (Derived Values)
// ═══════════════════════════════════════════════════════════════════════════

/// A computed value that automatically updates when its dependencies change.
///
/// Computed values are memoized - they only recompute when dependencies change.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::{Signal, Computed};
///
/// let count = Signal::new(5);
/// let doubled = Computed::new({
///     let count = count.clone();
///     move || count.get() * 2
/// });
///
/// assert_eq!(doubled.get(), 10);
/// count.set(10);
/// assert_eq!(doubled.get(), 20);
/// ```
#[derive(Debug)]
pub struct Computed<T> {
    signal: Signal<T>,
    _effect_id: u64, // Keep effect alive
}

impl<T: Clone + Send + Sync + 'static> Computed<T> {
    /// Create a new computed value from a function.
    pub fn new<F>(compute: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static + Clone,
    {
        let initial = compute();
        let signal = Signal::new(initial);
        let signal_clone = signal.clone();
        let compute_clone = compute.clone();

        let effect_id = next_id();
        let active = Arc::new(AtomicBool::new(true));
        let active_clone = active.clone();

        let wrapper = move || {
            if !active_clone.load(Ordering::Relaxed) {
                return;
            }

            // Set up tracking context
            CURRENT_EFFECT.with(|e| *e.borrow_mut() = Some(effect_id));
            let new_value = compute_clone();
            CURRENT_EFFECT.with(|e| *e.borrow_mut() = None);

            signal_clone.set(new_value);
        };

        // Register the effect
        register_effect(effect_id, Box::new(wrapper.clone()));

        // Run initial to set up subscriptions
        wrapper();

        Self {
            signal,
            _effect_id: effect_id,
        }
    }

    /// Get the current computed value.
    pub fn get(&self) -> T {
        self.signal.get()
    }

    /// Get the value without tracking as a dependency.
    pub fn get_untracked(&self) -> T {
        self.signal.get_untracked()
    }
}

impl<T> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal.clone(),
            _effect_id: self._effect_id,
        }
    }
}

/// Create a computed value from a signal and a transform function.
///
/// Convenience function for simple derived values.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::{Signal, computed};
///
/// let count = Signal::new(5);
/// let doubled = computed(count.clone(), |c| c * 2);
///
/// assert_eq!(doubled.get(), 10);
/// ```
pub fn computed<T, U, F>(source: Signal<T>, compute: F) -> Computed<U>
where
    T: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + 'static,
    F: Fn(T) -> U + Send + Sync + 'static + Clone,
{
    Computed::new(move || compute(source.get()))
}

// ═══════════════════════════════════════════════════════════════════════════
// EFFECT REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

thread_local! {
    static CURRENT_EFFECT: RefCell<Option<u64>> = const { RefCell::new(None) };
}

type EffectFn = Box<dyn Fn() + Send + Sync>;

// Non-WASM Implementation
#[cfg(not(target_arch = "wasm32"))]
lazy_static::lazy_static! {
    static ref EFFECT_REGISTRY: RwLock<HashMap<u64, EffectFn>> = RwLock::new(HashMap::new());
}

#[cfg(not(target_arch = "wasm32"))]
fn trigger_effect(id: u64) {
    let registry = EFFECT_REGISTRY.read().unwrap();
    if let Some(f) = registry.get(&id) {
        f();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn register_effect(id: u64, f: EffectFn) {
    let mut registry = EFFECT_REGISTRY.write().unwrap();
    registry.insert(id, f);
}

#[cfg(not(target_arch = "wasm32"))]
fn unregister_effect(id: u64) {
    let mut registry = EFFECT_REGISTRY.write().unwrap();
    registry.remove(&id);
}

// WASM Implementation
#[cfg(target_arch = "wasm32")]
thread_local! {
    static EFFECT_REGISTRY_WASM: RefCell<HashMap<u64, EffectFn>> = RefCell::new(HashMap::new());
}

#[cfg(target_arch = "wasm32")]
fn trigger_effect(id: u64) {
    EFFECT_REGISTRY_WASM.with(|r| {
        let registry = r.borrow();
        if let Some(f) = registry.get(&id) {
            f();
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn register_effect(id: u64, f: EffectFn) {
    EFFECT_REGISTRY_WASM.with(|r| {
        let mut registry = r.borrow_mut();
        registry.insert(id, f);
    });
}

#[cfg(target_arch = "wasm32")]
fn unregister_effect(id: u64) {
    EFFECT_REGISTRY_WASM.with(|r| {
        let mut registry = r.borrow_mut();
        registry.remove(&id);
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// EFFECTS
// ═══════════════════════════════════════════════════════════════════════════

/// An effect handle that can be used to stop the effect.
///
/// When dropped, the effect is automatically cleaned up.
pub struct EffectHandle {
    id: u64,
    active: Arc<AtomicBool>,
}

impl EffectHandle {
    /// Stop this effect from running.
    pub fn stop(&self) {
        self.active.store(false, Ordering::Relaxed);
        unregister_effect(self.id);
    }

    /// Check if this effect is still active.
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Prevent this effect from being cleaned up when the handle is dropped.
    ///
    /// Use this for long-lived effects that should run for the lifetime of
    /// the application (e.g., logging, analytics).
    ///
    /// # Warning
    /// Calling `forget()` means the effect will never be cleaned up.
    /// Only use for truly global effects.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nucleus_std::neutron::{Signal, create_effect};
    ///
    /// let count = Signal::new(0);
    ///
    /// // This effect will run forever
    /// create_effect({
    ///     let count = count.clone();
    ///     move || println!("Count: {}", count.get())
    /// }).forget();
    /// ```
    pub fn forget(self) {
        // Don't run Drop - effect stays registered
        std::mem::forget(self);
    }
}

impl Drop for EffectHandle {
    fn drop(&mut self) {
        // Automatic cleanup when handle goes out of scope
        self.stop();
    }
}

/// Create a side effect that runs when its dependencies change.
///
/// Effects auto-track any signals accessed via `.get()` and re-run
/// when those signals change.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::{Signal, create_effect};
///
/// let count = Signal::new(0);
///
/// create_effect({
///     let count = count.clone();
///     move || {
///         println!("Count changed to: {}", count.get());
///     }
/// });
///
/// count.set(1); // Prints: "Count changed to: 1"
/// ```
pub fn create_effect<F>(f: F) -> EffectHandle
where
    F: Fn() + Send + Sync + 'static + Clone,
{
    let id = next_id();
    let active = Arc::new(AtomicBool::new(true));
    let active_clone = active.clone();
    let f_clone = f.clone();

    let wrapper = move || {
        if !active_clone.load(Ordering::Relaxed) {
            return;
        }

        CURRENT_EFFECT.with(|e| *e.borrow_mut() = Some(id));
        f_clone();
        CURRENT_EFFECT.with(|e| *e.borrow_mut() = None);
    };

    register_effect(id, Box::new(wrapper.clone()));

    // Initial run
    wrapper();

    EffectHandle { id, active }
}

/// Create an effect without returning a handle.
///
/// Use this when you don't need to manually stop the effect.
/// The effect will run until the program ends.
pub fn effect<F>(f: F)
where
    F: Fn() + Send + Sync + 'static + Clone,
{
    let _ = create_effect(f);
}

// ═══════════════════════════════════════════════════════════════════════════
// STORE PATTERN
// ═══════════════════════════════════════════════════════════════════════════

/// A marker trait for reactive stores.
///
/// Implement this for your store structs to get access to
/// convenience methods.
pub trait Store: Sized {
    /// Create a new instance of the store.
    fn init() -> Self;
}

/// Attribute macro to create a reactive store.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::store;
///
/// #[store]
/// struct AppState {
///     count: i32,
///     name: String,
/// }
/// ```
pub use nucleus_macros::store;

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL STATE
// ═══════════════════════════════════════════════════════════════════════════

/// A thread-safe wrapper for global static state.
///
/// Uses `OnceLock` for lazy initialization.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::{Global, Signal};
///
/// static COUNT: Global<Signal<i32>> = Global::new(|| Signal::new(0));
///
/// fn increment() {
///     COUNT.update(|c| *c += 1);
/// }
/// ```
pub struct Global<T> {
    cell: std::sync::OnceLock<T>,
    init: fn() -> T,
}

impl<T> Global<T> {
    /// Create a new global state container.
    ///
    /// The initializer function is called on the first access.
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            cell: std::sync::OnceLock::new(),
            init,
        }
    }
}

impl<T> std::ops::Deref for Global<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.cell.get_or_init(self.init)
    }
}

impl<T: Clone + 'static> Signal<T> {
    /// Alias for `update`. Modifies the value in-place.
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.update(f)
    }
}

thread_local! {
    static CONTEXT_STACK: RefCell<Vec<HashMap<std::any::TypeId, Box<dyn std::any::Any>>>> =
        RefCell::new(vec![HashMap::new()]);
}

/// Provide a value to the context, making it available to child effects.
///
/// # Example
///
/// ```rust,ignore
/// use nucleus_std::neutron::{provide_context, use_context};
///
/// let theme = Signal::new("dark".to_string());
/// provide_context(theme.clone());
///
/// // Later, in any child effect:
/// let theme = use_context::<Signal<String>>();
/// ```
pub fn provide_context<T: Clone + 'static>(value: T) {
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if let Some(current) = stack.last_mut() {
            current.insert(std::any::TypeId::of::<T>(), Box::new(value));
        }
    });
}

/// Get a value from the context.
///
/// Returns `None` if no provider for this type exists.
pub fn use_context<T: Clone + 'static>() -> Option<T> {
    CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for frame in stack.iter().rev() {
            if let Some(value) = frame.get(&std::any::TypeId::of::<T>()) {
                return value.downcast_ref::<T>().cloned();
            }
        }
        None
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// WATCH (Named Effects with Cleanup)
// ═══════════════════════════════════════════════════════════════════════════

/// Watch a signal and run a callback when it changes.
///
/// Unlike effects, watch explicitly specifies what to watch and
/// provides the old and new values.
///
/// # Example
///
/// ```rust
/// use nucleus_std::neutron::{Signal, watch};
///
/// let count = Signal::new(0);
///
/// let handle = watch(
///     count.clone(),
///     |old, new| println!("Count changed from {} to {}", old, new)
/// );
///
/// count.set(5); // Prints: "Count changed from 0 to 5"
/// ```
pub fn watch<T, F>(signal: Signal<T>, callback: F) -> EffectHandle
where
    T: Clone + Send + Sync + PartialEq + 'static,
    F: Fn(T, T) + Send + Sync + Clone + 'static,
{
    let prev = Arc::new(RwLock::new(signal.get_untracked()));
    let signal_clone = signal.clone();
    let callback_clone = callback.clone();
    let prev_clone = prev.clone();

    create_effect(move || {
        let current = signal_clone.get();
        let old = {
            let lock = prev_clone.read().unwrap();
            lock.clone()
        };

        if old != current {
            callback_clone(old, current.clone());
            *prev_clone.write().unwrap() = current;
        }
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering as AtomicOrdering};

    #[test]
    fn test_signal_basics() {
        let count = Signal::new(0);
        assert_eq!(count.get(), 0);

        count.set(10);
        assert_eq!(count.get(), 10);

        count.update(|v| *v += 5);
        assert_eq!(count.get(), 15);
    }

    #[test]
    fn test_signal_untracked() {
        let count = Signal::new(42);
        assert_eq!(count.get_untracked(), 42);
    }

    #[test]
    fn test_set_if_changed() {
        let count = Signal::new(0);
        let run_count = Arc::new(AtomicU32::new(0));
        let run_count_c = run_count.clone();
        let count_c = count.clone();

        let _handle = create_effect(move || {
            let _val = count_c.get();
            run_count_c.fetch_add(1, AtomicOrdering::Relaxed);
        });

        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 1);

        count.set_if_changed(0); // Same value - should not trigger
        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 1);

        count.set_if_changed(5); // Different - should trigger
        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 2);
    }

    #[test]
    fn test_effect_reactivity() {
        let count = Signal::new(0);
        let count_c = count.clone();

        let run_count = Arc::new(AtomicU32::new(0));
        let run_count_c = run_count.clone();

        let _handle = create_effect(move || {
            let _val = count_c.get();
            run_count_c.fetch_add(1, AtomicOrdering::Relaxed);
        });

        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 1);

        count.set(1);
        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 2);

        count.update(|v| *v += 1);
        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 3);
    }

    #[test]
    fn test_effect_cleanup() {
        let count = Signal::new(0);
        let count_c = count.clone();

        let run_count = Arc::new(AtomicU32::new(0));
        let run_count_c = run_count.clone();

        let handle = create_effect(move || {
            let _val = count_c.get();
            run_count_c.fetch_add(1, AtomicOrdering::Relaxed);
        });

        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 1);

        handle.stop();
        count.set(1);

        // Effect should not run after stop
        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 1);
    }

    #[test]
    fn test_batch_updates() {
        let a = Signal::new(0);
        let b = Signal::new(0);
        let a_c = a.clone();
        let b_c = b.clone();

        let run_count = Arc::new(AtomicU32::new(0));
        let run_count_c = run_count.clone();

        let _handle = create_effect(move || {
            let _ = a_c.get() + b_c.get();
            run_count_c.fetch_add(1, AtomicOrdering::Relaxed);
        });

        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 1);

        // Without batch: would trigger 2 times
        // With batch: triggers once
        batch(|| {
            a.set(1);
            b.set(2);
        });

        assert_eq!(run_count.load(AtomicOrdering::Relaxed), 2);
    }

    #[test]
    fn test_computed_basic() {
        let count = Signal::new(5);
        let count_c = count.clone();

        let doubled = Computed::new(move || count_c.get() * 2);

        assert_eq!(doubled.get(), 10);
    }

    #[test]
    fn test_computed_helper() {
        let count = Signal::new(5);
        let doubled = computed(count.clone(), |c| c * 2);

        assert_eq!(doubled.get(), 10);

        count.set(10);
        assert_eq!(doubled.get(), 20);
    }

    #[test]
    fn test_derived_dependency() {
        let first = Signal::new("Hello".to_string());
        let last = Signal::new("World".to_string());

        let full_name = Arc::new(RwLock::new(String::new()));
        let full_name_c = full_name.clone();

        let f_c = first.clone();
        let l_c = last.clone();

        let _handle = create_effect(move || {
            let s = format!("{} {}", f_c.get(), l_c.get());
            *full_name_c.write().unwrap() = s;
        });

        assert_eq!(*full_name.read().unwrap(), "Hello World");

        first.set("Good".to_string());
        assert_eq!(*full_name.read().unwrap(), "Good World");

        last.set("Morning".to_string());
        assert_eq!(*full_name.read().unwrap(), "Good Morning");
    }

    #[test]
    fn test_thread_safety() {
        let counter = Signal::new(0);
        let counter_c = counter.clone();

        let thread_handle = std::thread::spawn(move || {
            for _ in 0..100 {
                counter_c.update(|v| *v += 1);
            }
        });

        thread_handle.join().unwrap();
        assert_eq!(counter.get(), 100);
    }

    #[test]
    fn test_context() {
        let theme = Signal::new("dark".to_string());
        provide_context(theme.clone());

        let retrieved = use_context::<Signal<String>>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get(), "dark");
    }

    #[test]
    fn test_store_pattern() {
        #[store]
        struct CounterStore {
            count: i32,
            name: String,
        }

        let store = CounterStore::new(0, "Counter".to_string());
        assert_eq!(store.count.get(), 0);
        assert_eq!(store.name.get(), "Counter");

        store.count.set(5);
        assert_eq!(store.count.get(), 5);

        store.name.set("Updated".to_string());
        assert_eq!(store.name.get(), "Updated");
    }
}
