use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use ahash::AHashMap;
use std::path::Path;
use std::sync::Arc;
use axum::body::Bytes;
use tokio::sync::mpsc;
// use std::fs;
use arc_swap::ArcSwap;

pub struct HotSwapListener {
    routes: Arc<ArcSwap<AHashMap<String, Bytes>>>,
    tx: tokio::sync::broadcast::Sender<String>,
}

impl HotSwapListener {
    pub fn new(routes: Arc<ArcSwap<AHashMap<String, Bytes>>>, tx: tokio::sync::broadcast::Sender<String>) -> Self {
        Self { routes, tx }
    }

    pub async fn listen(&mut self) {
        let (tx, mut rx) = mpsc::channel(1);
        
        let mut watcher = RecommendedWatcher::new(move |res| {
            tx.blocking_send(res).ok();
        }, Config::default()).unwrap();

        let views_path = Path::new("src/views");
        if views_path.exists() {
             // Watch the view directory
             if let Err(e) = watcher.watch(views_path, RecursiveMode::Recursive) {
                 eprintln!("⚠️ Atom: Failed to watch src/views: {}", e);
                 return;
             }
             println!("🔥 Atom: Hot Swap enabled. Watching src/views for changes...");
        } else {
             return;
        }

        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    // Only care about Write/Create/Remove
                    if event.kind.is_modify() || event.kind.is_create() {
                        for path in event.paths {
                            if path.extension().is_some_and(|e| e == "ncl") {
                                self.reload_file(&path).await;
                            }
                        }
                    }
                }
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        }
    }

    async fn reload_file(&self, path: &Path) {
        if let Ok(content) = tokio::fs::read_to_string(path).await {
             let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
             println!("🔄 Reloading {}...", stem);
             
             // Re-compile
             if let Ok((_, nodes)) = ncc::parse_root(&content) {
                 let html = ncc::render_html(&nodes);
                 
                 // Update Routing Table (RCU Pattern)
                 // 1. Get current map
                 let current = self.routes.load();
                 // 2. Clone it (New allocation)
                 let mut new_map = (**current).clone(); 
                 // 3. Update new map
                 new_map.insert(stem, Bytes::from(html));
                 // 4. Atomic Store
                 self.routes.store(Arc::new(new_map));
                     
                 // Broadcast Reload!
                 let _ = self.tx.send("hmr:reload".to_string());
                 println!("✅ Component hot-swapped.");
             } else {
                 eprintln!("❌ Failed to parse {}", stem);
             }
        }
    }
}
