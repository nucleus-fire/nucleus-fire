#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate self as nucleus_std;

pub mod errors;
pub mod config;
pub mod photon;
pub mod fortress;
pub mod neutron;
pub mod vault;
pub mod postman;
pub mod pulse;
pub mod gondola;
pub mod polyglot;
pub mod sonar;
pub mod lens;
pub mod chain;
pub mod stream;
pub mod devtools;
pub mod payments;
pub mod rpc;
pub mod federation;
pub mod neural;
pub mod browser;
pub mod forms;
pub mod oauth;
pub mod beacon;
pub mod middleware;
pub mod upload;
pub mod cache;
pub mod logging;
pub mod scheduler;
pub mod session;
pub mod redis_cache;
pub mod health;
pub mod pool_monitor;
pub mod utils;
pub mod scout;
pub mod testing;
pub mod push;
pub mod graph;
pub mod tenant;

// Re-export attribute macros
pub use nucleus_macros::server;

pub mod mcp;
pub mod agent;

// Re-exports
pub use fortress::Fortress;
pub use fortress::{RateLimiter, RateLimitConfig, RateLimitResult};
pub use fortress::{AuthUser, OptionalAuth, require_auth};
pub use vault::{Money, Ledger, Account, AccountType, Transaction, LedgerEntry, Vault};
pub use sonar::Sonar;
pub use pulse::{Pulse, Job, JobConfig, JobPriority, JobStatus, PulseError};
pub use postman::Postman;
pub use lens::Lens;
pub use polyglot::Polyglot;
pub use photon::{init_db, db, Op, Model, Builder};
pub use config::{Config, GLOBAL_CONFIG};
pub use chain::Chain;
pub use payments::Stripe;
pub use neural::{Neural, NeuralError, ChatMessage, Role, Usage};
pub use browser::{Browser, BrowserOptions, BrowserError};
pub use neutron::Signal;
pub use beacon::Beacon;
pub use upload::{Upload, UploadConfig, UploadedFile, UploadError};
pub use cache::{Cache, CacheKey, cached, cached_with_ttl};
pub use logging::{init as init_logging, LogConfig, LogLevel, LogFormat};
pub use scheduler::{Scheduler, CronSchedule, CronError};
pub use session::{Session, SessionManager, SessionConfig, SessionStore, MemorySessionStore, SameSite};
pub use stream::{StreamHub, WebSocket, SocketMessage, StreamHandler, StreamError, Room};
pub use redis_cache::{UnifiedCache, CacheBackend, MemoryCacheBackend, RedisBackend, RedisCacheError, memory_cache, redis_cache as create_redis_cache};
pub use health::{HealthChecker, HealthReport, HealthStatus, ComponentCheck};
pub use pool_monitor::{PoolMonitor, PoolStats, PoolHealth, PoolHealthStatus, PoolDashboard, QueryMetrics, PoolSizingRecommendation};
pub use scout::{Scout, ScoutError, SearchQuery, SearchResults, SearchHit, IndexInfo, IndexTask, TaskStatus, SearchParams};
pub use tenant::{Tenant, TenantExtractor, TenantStrategy, TenantGuard, TenantInfo, TenantError, TenantQuery};
pub use sqlx;
pub use axum;

#[cfg(test)]
mod neutron_store_tests;

#[cfg(test)]
mod edge_tests;
