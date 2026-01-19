#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate self as nucleus_std;

pub mod beacon;
#[cfg(feature = "browser")]
pub mod browser;
pub mod cache;
pub mod chain;
pub mod config;
pub mod devtools;
pub mod errors;
pub mod federation;
pub mod forms;
pub mod fortress;
pub mod gondola;
#[cfg(feature = "graphql")]
pub mod graph;
pub mod health;
pub mod lens;
pub mod logging;
pub mod middleware;
pub mod neural;
pub mod neutron;
pub mod oauth;
pub mod payments;
pub mod photon;
pub mod polyglot;
pub mod pool_monitor;
#[cfg(feature = "mail")]
pub mod postman;
pub mod pulse;
pub mod push;
pub mod redis_cache;
pub mod rpc;
pub mod scheduler;
pub mod scout;
pub mod session;
pub mod sonar;
pub mod stream;
pub mod tenant;
pub mod testing;
pub mod upload;
pub mod utils;
pub mod vault;

// Re-export attribute macros
pub use nucleus_macros::server;

#[cfg(feature = "ai")]
pub mod agent;
#[cfg(feature = "ai")]
pub mod mcp;

// Re-exports
pub use axum;
pub use beacon::Beacon;
#[cfg(feature = "browser")]
pub use browser::{Browser, BrowserError, BrowserOptions};
pub use cache::{cached, cached_with_ttl, Cache, CacheKey};
pub use chain::Chain;
pub use config::{Config, GLOBAL_CONFIG};
pub use fortress::Fortress;
pub use fortress::{require_auth, AuthUser, OptionalAuth};
pub use fortress::{RateLimitConfig, RateLimitResult, RateLimiter};
pub use health::{ComponentCheck, HealthChecker, HealthReport, HealthStatus};
pub use lens::Lens;
pub use logging::{init as init_logging, LogConfig, LogFormat, LogLevel};
pub use neural::{ChatMessage, Neural, NeuralError, Role, Usage};
pub use neutron::Signal;
pub use payments::Stripe;
pub use photon::{db, init_db, Builder, Model, Op};
pub use polyglot::Polyglot;
pub use pool_monitor::{
    PoolDashboard, PoolHealth, PoolHealthStatus, PoolMonitor, PoolSizingRecommendation, PoolStats,
    QueryMetrics,
};
#[cfg(feature = "mail")]
pub use postman::Postman;
pub use pulse::{Job, JobConfig, JobPriority, JobStatus, Pulse, PulseError};
pub use redis_cache::{
    memory_cache, redis_cache as create_redis_cache, CacheBackend, MemoryCacheBackend,
    RedisBackend, RedisCacheError, UnifiedCache,
};
pub use scheduler::{CronError, CronSchedule, Scheduler};
pub use scout::{
    IndexInfo, IndexTask, Scout, ScoutError, SearchHit, SearchParams, SearchQuery, SearchResults,
    TaskStatus,
};
pub use session::{
    MemorySessionStore, SameSite, Session, SessionConfig, SessionManager, SessionStore,
};
pub use sonar::Sonar;
pub use sqlx;
pub use stream::{Room, SocketMessage, StreamError, StreamHandler, StreamHub, WebSocket};
pub use tenant::{
    Tenant, TenantError, TenantExtractor, TenantGuard, TenantInfo, TenantQuery, TenantStrategy,
};
pub use upload::{Upload, UploadConfig, UploadError, UploadedFile};
pub use vault::{Account, AccountType, Ledger, LedgerEntry, Money, Transaction, Vault};

#[cfg(test)]
mod neutron_store_tests;

#[cfg(test)]
mod edge_tests;
