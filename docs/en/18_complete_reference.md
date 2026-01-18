# Nucleus Framework: Complete Reference

> **Version**: 3.5  
> **Modules**: 42  
> **Status**: Living Document

This is the comprehensive reference for all Nucleus framework features and modules.

---

## Quick Navigation

| Category | Modules |
|----------|---------|
| [Core Guides](#core-guides) | Getting Started, Tutorial, Concepts, Structure |
| [Data & Database](#data--database) | Photon, Config, Sonar, Scout |
| [Security & Auth](#security--authentication) | Fortress, OAuth, Session |
| [State & Reactivity](#state--reactivity) | Neutron, Gondola |
| [Backend Services](#backend-services) | Pulse, Scheduler, Postman, Upload, Logging |
| [Real-time](#real-time) | Stream, DevTools |
| [Commerce](#commerce) | Payments, Vault |
| [Integrations](#integrations) | Federation, Neural, Browser, MCP, Agent |
| [I18n & Media](#i18n--media) | Polyglot, Lens, Beacon |
| [Infrastructure](#infrastructure) | Health, Pool Monitor, Cache, Redis Cache, Tenant |
| [API & Communication](#api--communication) | RPC, Graph, Push, Forms, Middleware |

---

## Core Guides

| Topic | Description | Link |
|-------|-------------|------|
| **Getting Started** | Installation & First App | [Getting Started](#01_getting_started) |
| **Tutorial** | Step-by-step Quick Start | [Quick Start Tutorial](#24_quick_start_tutorial) |
| **Core Concepts** | Mental Model & Architecture | [Core Concepts](#02_core_concepts) |
| **Project Structure** | Anatomy of a Nucleus App | [Project Structure](#25_project_structure) |

---

## Standard Library (42 Modules)

### Data & Database

| Module | Description | Guide |
|--------|-------------|-------|
| **Photon** | Type-safe SQL ORM with migrations, relationships, query builder | [Database Guide](#20_database_guide) |
| **Config** | Environment configuration, TOML parsing, secrets | [Configuration](#configuration) |
| **Sonar** | Embedded full-text search with BM25 ranking | [Search Guide](#47_search_guide) |
| **Scout** | Advanced search with indexing, facets, and async tasks | [Scout Guide](#55_scout_guide) |

---

### Security & Authentication

| Module | Description | Guide |
|--------|-------------|-------|
| **Fortress** | Argon2 hashing, JWT tokens, RBAC, rate limiting | [Authentication Guide](#21_authentication_guide) |
| **OAuth** | Social login (Google, GitHub, Discord, Twitter) | [Social Login Guide](#27_social_login_guide) |
| **Session** | Cookie sessions, flash messages, CSRF protection | [Session Guide](#36_session_guide) |

---

### State & Reactivity

| Module | Description | Guide |
|--------|-------------|-------|
| **Neutron** | Fine-grained reactivity with `Signal<T>` and `Store` | [State Management](#15_state_management) |
| **Gondola** | Merkle-tree based offline sync with CRDTs | [Offline Sync Guide](#27_offline_sync_guide) |

---

### Backend Services

| Module | Description | Guide |
|--------|-------------|-------|
| **Pulse** | Persistent background jobs with retries & dead letter queue | [Job Queue Guide](#34_job_queue_guide) |
| **Scheduler** | Recurring tasks with cron expressions | [Scheduled Jobs Guide](#35_scheduled_jobs_guide) |
| **Postman** | Transactional email (SMTP & AWS SES) | [Email Guide](#25_email_guide) |
| **Upload** | Multipart file handling, validation, secure storage | [File Upload Guide](#45_upload_guide) |
| **Logging** | Structured logging with tracing, JSON/pretty output | [Logging Guide](#46_logging_guide) |

---

### Real-time

| Module | Description | Guide |
|--------|-------------|-------|
| **Stream** | WebSocket rooms, broadcast, presence tracking | [WebSocket Guide](#30_websocket_guide), [Rooms Guide](#38_websocket_rooms_guide) |
| **DevTools** | Debug utilities, request inspection | [Developer Tools](#51_developer_tools_guide) |

---

### Commerce

| Module | Description | Guide |
|--------|-------------|-------|
| **Payments** | Stripe integration (customers, subscriptions, webhooks) | [Payments Guide](#49_payments_guide) |
| **Vault** | Double-entry ledger, Money type, financial transactions | [Vault Finance Guide](#48_vault_finance_guide) |
| **Chain** | Web3/EVM utilities, wallet connections | [Crypto Guide](#50_crypto_chain_guide) |

---

### Integrations

| Module | Description | Guide |
|--------|-------------|-------|
| **Federation** | CMS aggregation (Directus, Sanity, Strapi) | [Federation Guide](#25_federation_guide) |
| **Neural** | AI/LLM client (OpenAI-compatible APIs) | [Neural AI Guide](#42_neural_ai_guide) |
| **Browser** | Headless Chrome automation | [Browser Automation](#43_browser_automation_guide) |
| **MCP** | Model Context Protocol server implementation | [MCP Protocol](#60_mcp_protocol) |
| **Agent** | AI Agent framework with tool use | [AI Agents Guide](#59_ai_agents) |

---

### I18n & Media

| Module | Description | Guide |
|--------|-------------|-------|
| **Polyglot** | Internationalization, locale detection, translations | [i18n Guide](#28_i18n_guide) |
| **Lens** | Image processing (resize, crop, convert, optimize) | [Image Processing](#26_image_processing_guide) |
| **Beacon** | Privacy-first analytics, event tracking | [Analytics Guide](#29_analytics_guide) |

---

### Infrastructure

| Module | Description | Guide |
|--------|-------------|-------|
| **Health** | Liveness/readiness probes, component checks | [Health Check Guide](#39_health_check_guide) |
| **Pool Monitor** | Connection pool stats, slow query detection | [Pool Monitor Guide](#41_pool_monitor_guide) |
| **Cache** | In-memory cache with TTL & pattern invalidation | [Cache Guide](#44_cache_guide) |
| **Redis Cache** | Distributed Redis-backed caching | [Redis Cache Guide](#37_redis_cache_guide) |
| **Tenant** | Multi-tenancy with subdomain/header/path strategies | [Multi-tenancy Guide](#50_multi_tenancy_guide) |

---

### API & Communication

| Module | Description | Guide |
|--------|-------------|-------|
| **RPC** | Server functions with `#[server]` macro | [API Development](#22_api_development) |
| **Graph** | GraphQL server implementation | [GraphQL Guide](#58_graphql_guide) |
| **Push** | Web push notifications (VAPID) | [Push Notifications](#57_push_notifications_guide) |
| **Forms** | Form validation, CSRF, file uploads | [Forms & Validation](#07_forms_and_validation) |
| **Middleware** | Request/response pipeline, guards | [Middleware Guide](#32_middleware_guide) |
| **Errors** | Miette-powered error handling | [Error Handling](#33_error_handling_guide) |
| **Testing** | Test utilities, mocks, fixtures | [Testing Guide](#26_testing_guide) |
| **Utils** | Common utilities and helpers | - |

---

## Development References

| Topic | Description | Link |
|-------|-------------|------|
| **Syntax (NCL)** | Tags, Attributes, Loops, Logic | [Syntax Reference](#19_syntax_reference) |
| **CLI** | `nucleus` command reference | [CLI Reference](#17_cli_reference) |
| **Compiler** | NCC compiler internals | [Compiler Reference](#03_compiler_reference) |
| **Components** | Props, slots, scoped CSS | [Components Guide](#26_components_guide) |
| **Client Navigation** | SPA routing with n:link | [Navigation Guide](#52_client_navigation_guide) |

---

## Advanced Topics

| Topic | Description | Link |
|-------|-------------|------|
| **Deployment** | Docker, VPS, Cloud platforms | [Deployment Guide](#23_deployment_guide) |
| **Static Export** | SSG & publishing | [Static Export](#31_static_export_guide) |
| **Performance** | Benchmarks & optimization | [Benchmarks](#13_performance_benchmarks) |
| **Architecture** | Internal design | [Architecture](#13_architecture_improvements) |
| **Type Safety** | End-to-end type checking | [Type Safety](#type_safety) |
| **Web Standards** | RFC compliance | [Web Standards](#web_standards) |

---

## Quick Import Reference

```rust
use nucleus_std::{
    // Database
    photon::{db, init_db, Op, Model, Builder},
    Config,
    Sonar,
    Scout,
    
    // Security
    Fortress,
    RateLimiter,
    AuthUser,
    Session,
    
    // State
    Signal,
    
    // Backend
    Pulse, Job,
    Scheduler,
    Postman,
    Upload,
    
    // Real-time
    StreamHub, WebSocket,
    
    // Commerce
    Stripe,
    Vault, Money, Ledger,
    Chain,
    
    // AI
    Neural,
    Browser,
    
    // Infrastructure
    HealthChecker,
    PoolMonitor,
    Cache,
    Tenant,
};
```

---

> **Note**: For the absolute latest API signatures, always check the source code in `crates/nucleus-std/src/lib.rs`.
