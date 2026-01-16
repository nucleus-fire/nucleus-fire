
#[cfg(test)]
mod tests {
    use crate::neutron::{Signal, Global, store};

    #[test]
    fn test_global_state() {
        static GLOBAL_COUNT: Global<Signal<i32>> = Global::new(|| Signal::new(0));

        assert_eq!(GLOBAL_COUNT.get(), 0);
        GLOBAL_COUNT.set(10);
        assert_eq!(GLOBAL_COUNT.get(), 10);
        GLOBAL_COUNT.modify(|c| *c += 5);
        assert_eq!(GLOBAL_COUNT.get(), 15);
    }

    #[test]
    fn test_modify_alias() {
        let sig = Signal::new(vec![1, 2, 3]);
        sig.modify(|v| v.push(4));
        assert_eq!(sig.get(), vec![1, 2, 3, 4]);
    }

    #[store]
    struct TestStore {
        count: i32,
        label: String,
    }

    #[test]
    fn test_store_macro_generated() {
        // Test constructor
        let s = TestStore::new(42, "hello".to_string());
        assert_eq!(s.count.get(), 42);
        assert_eq!(s.label.get(), "hello");

        // Test reactivity
        s.count.set(100);
        assert_eq!(s.count.get(), 100);

        // Test Default logic (via init)
        // Wait, for i32 default is 0, string is ""
        // Usage of Default
        let s_def = TestStore::default();
        assert_eq!(s_def.count.get(), 0);
        assert_eq!(s_def.label.get(), "");
    }
}
