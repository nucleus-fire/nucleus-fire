<n:script>
    use nucleus_std::gondola::{SyncStore, LWWRegister, GCounter, PNCounter, Gondola, Row};

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 1: Mobile Sync Endpoint
    // ═══════════════════════════════════════════════════════════════════════════

    async fn sync_with_server(client_root: &str, server_rows: Vec<Row>) -> Vec<String> {
        // Compare Merkle roots - if they match, no sync needed
        let changed_ids = Gondola::diff_trees(client_root, &server_rows);
        
        if changed_ids.is_empty() {
            println!("Client is in sync!");
        } else {
            println!("{} items need syncing", changed_ids.len());
        }
        
        changed_ids
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 2: Offline-First Key-Value Store
    // ═══════════════════════════════════════════════════════════════════════════

    fn setup_offline_store() -> SyncStore {
        let mut store = SyncStore::with_node_id("device_12345");
        
        // Store user preferences
        store.set("user.theme", "dark");
        store.set("user.notifications", true);
        store.set("user.language", "en");
        
        // Store can be serialized and persisted to localStorage/SQLite
        println!("Store version: {}", store.version());
        println!("Merkle root: {}", store.merkle_root());
        
        store
    }

    fn sync_stores(local: &mut SyncStore, remote: &SyncStore) {
        // Merge remote changes - conflicts resolved by timestamp
        local.merge(remote);
        println!("Synced! New version: {}", local.version());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 3: Collaborative Counters (Likes/Views)
    // ═══════════════════════════════════════════════════════════════════════════

    struct PostStats {
        views: GCounter,
        likes: GCounter,
    }

    impl PostStats {
        fn new() -> Self {
            Self {
                views: GCounter::with_node_id("server"),
                likes: GCounter::with_node_id("server"),
            }
        }

        fn view(&mut self) {
            self.views.increment();
        }

        fn like(&mut self) {
            self.likes.increment();
        }

        fn merge_from(&mut self, other: &PostStats) {
            self.views.merge(&other.views);
            self.likes.merge(&other.likes);
        }

        fn display(&self) -> String {
            format!("{} views, {} likes", self.views.value(), self.likes.value())
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 4: Inventory with PN-Counter
    // ═══════════════════════════════════════════════════════════════════════════

    struct Inventory {
        items: std::collections::HashMap<String, PNCounter>,
    }

    impl Inventory {
        fn new() -> Self {
            Self { items: std::collections::HashMap::new() }
        }

        fn add_stock(&mut self, product_id: &str, quantity: u32) {
            let counter = self.items.entry(product_id.to_string())
                .or_insert_with(PNCounter::new);
            for _ in 0..quantity {
                counter.increment();
            }
        }

        fn remove_stock(&mut self, product_id: &str, quantity: u32) {
            if let Some(counter) = self.items.get_mut(product_id) {
                for _ in 0..quantity {
                    counter.decrement();
                }
            }
        }

        fn get_stock(&self, product_id: &str) -> i64 {
            self.items.get(product_id)
                .map(|c| c.value())
                .unwrap_or(0)
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 5: Last-Write-Wins Settings
    // ═══════════════════════════════════════════════════════════════════════════

    struct UserSettings {
        theme: LWWRegister<String>,
        font_size: LWWRegister<u32>,
    }

    impl UserSettings {
        fn new() -> Self {
            Self {
                theme: LWWRegister::with_node_id("light".to_string(), "default"),
                font_size: LWWRegister::with_node_id(14, "default"),
            }
        }

        fn set_theme(&mut self, theme: &str) {
            self.theme.set(theme.to_string());
        }

        fn sync_with(&mut self, other: &UserSettings) {
            // Newer timestamp wins
            self.theme.merge(&other.theme);
            self.font_size.merge(&other.font_size);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 6: Full Sync Flow
    // ═══════════════════════════════════════════════════════════════════════════

    async fn full_sync_flow(local_store: &mut SyncStore, server_url: &str) -> Result<(), String> {
        // 1. Encode local changes
        let local_changes = local_store.encode_since(0);
        
        // 2. Send to server and get server changes (simulated)
        // let response = http_client.post(server_url).body(local_changes).send().await;
        // let server_changes = response.bytes();
        
        // 3. Apply server changes locally
        // local_store.apply(&server_changes)?;
        
        println!("Sync complete! Version: {}", local_store.version());
        Ok(())
    }
</n:script>
