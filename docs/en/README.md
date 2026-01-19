# Nucleus Framework Documentation

Welcome to the official documentation for Nucleus, the high-performance unified Rust web framework.

---

## 🚀 Start Here

| New to Nucleus? | Experienced Developer? |
|-----------------|------------------------|
| [Quick Start Tutorial](#24_quick_start_tutorial) | [Complete Reference](#18_complete_reference) |
| Build your first app in 5 minutes | Every API and feature documented |

---

## 📖 Learning Path

### 1. Fundamentals
*   [Quick Start Tutorial](#24_quick_start_tutorial) - Build your first app
*   [Project Structure](#25_project_structure) - What goes where
*   [Syntax Reference](#19_syntax_reference) - NCL tags and attributes
*   [Core Concepts](#02_core_concepts) - Reactor, HMR, architecture

### 2. Building Features
*   [Database Guide](#20_database_guide) - CRUD, migrations, relationships
*   [Authentication](#21_authentication_guide) - Login, sessions, RBAC
*   [API Development](#22_api_development) - REST endpoints
*   [State Management](#15_state_management) - Signals and Stores
*   [Forms & Validation](#07_forms_and_validation) - User input

### 3. Components & UI
*   [Components Guide](#26_components_guide) - **Reusable UI components**
*   [Props & Slots](26_components_guide.md#props-system) - Component data flow
*   [Scoped CSS](26_components_guide.md#scoped-css) - Style isolation
*   [Built-in Components](26_components_guide.md#built-in-components) - Button, Card, Badge

### 4. Integrations
*   [Federation Guide](#25_federation_guide) - CMS content aggregation
*   [Email Guide](#25_email_guide) - SMTP & AWS SES email
*   [Configuration](#configuration) - Environment & TOML config

### 5. Media & Processing
*   [Image Processing Guide](#26_image_processing_guide) - Resize, crop, convert
*   [Offline Sync Guide](#27_offline_sync_guide) - CRDTs for offline-first
*   [i18n Guide](#28_i18n_guide) - Internationalization
*   [Analytics Guide](#29_analytics_guide) - Privacy-first tracking

### 6. Production
*   [Testing Guide](#26_testing_guide) - Unit, integration, E2E
*   [Deployment & Hosting Guide](#23_deployment_guide) - Docker, PaaS, cloud
*   [Performance](#13_performance_benchmarks) - 20,000+ req/sec

---

## 📚 Reference

| Document | Description |
|----------|-------------|
| [Complete Reference](#18_complete_reference) | Every feature in one place |
| [CLI Reference](#17_cli_reference) | All commands |
| [Syntax Reference](#19_syntax_reference) | All NCL tags |
| [Stdlib Reference](#04_stdlib_reference) | All 19 modules |
| [Components Guide](#26_components_guide) | Props, slots, scoped CSS |
| [Feature List](#00_master_feature_list) | Capabilities overview |

---

## 🧩 Components

Nucleus includes a full component system for building reusable UI:

| Component | Props | Description |
|-----------|-------|-------------|
| `Button` | variant, size, href | Buttons and link buttons |
| `Card` | variant, glass | Container cards |
| `Badge` | variant, icon | Labels and tags |
| `FeatureCard` | icon, title, description | Feature showcase cards |
| `StatCard` | value, label, highlight | Metrics display |
| `RaceLane` | name, icon, logo | Racing visualization |
| `CodeBlock` | language | Syntax-highlighted code |

**Learn more:** [Components Guide](#26_components_guide)

---

## 📦 Standard Library (42 Modules)

| Module | Category | Description |
|--------|----------|-------------|
| [Photon](04_stdlib_reference.md#photon) | Data | Type-safe SQL ORM |
| [Config](04_stdlib_reference.md#config) | Data | Environment configuration |
| [Sonar](04_stdlib_reference.md#sonar) | Data | Embedded full-text search |
| [Scout](04_stdlib_reference.md#scout) | Data | Advanced search with indexing |
| [Fortress](04_stdlib_reference.md#fortress) | Security | Auth, JWT, RBAC, rate limiting |
| [OAuth](04_stdlib_reference.md#oauth) | Security | Social login (Google, GitHub, etc.) |
| [Session](04_stdlib_reference.md#session) | Security | Cookie sessions, CSRF |
| [Chain](04_stdlib_reference.md#chain) | Security | Web3/EVM utilities |
| [Neutron](04_stdlib_reference.md#neutron) | State | Reactive signals & stores |
| [Gondola](04_stdlib_reference.md#gondola) | State | CRDT offline sync |
| [Pulse](04_stdlib_reference.md#pulse) | Backend | Background job queue |
| [Scheduler](04_stdlib_reference.md#scheduler) | Backend | Cron/recurring tasks |
| [Postman](04_stdlib_reference.md#postman) | Backend | Email (SMTP & SES) |
| [Upload](04_stdlib_reference.md#upload) | Backend | File uploads & validation |
| [Logging](04_stdlib_reference.md#logging) | Backend | Structured logging |
| [RPC](04_stdlib_reference.md#rpc) | Backend | Server functions |
| [Payments](04_stdlib_reference.md#payments) | Commerce | Stripe integration |
| [Vault](04_stdlib_reference.md#vault) | Commerce | Money handling & ledger |
| [Stream](04_stdlib_reference.md#stream) | Real-time | WebSocket rooms & broadcast |
| [DevTools](04_stdlib_reference.md#devtools) | Real-time | Debug utilities |
| [Health](04_stdlib_reference.md#health) | Infrastructure | Liveness/readiness probes |
| [Pool Monitor](04_stdlib_reference.md#pool-monitor) | Infrastructure | DB connection pool stats |
| [Cache](04_stdlib_reference.md#cache) | Infrastructure | In-memory caching |
| [Redis Cache](04_stdlib_reference.md#redis-cache) | Infrastructure | Distributed caching |
| [Tenant](04_stdlib_reference.md#tenant) | Infrastructure | Multi-tenancy |
| [Federation](04_stdlib_reference.md#federation) | Integration | CMS aggregation |
| [Neural](04_stdlib_reference.md#neural) | AI | LLM/AI client |
| [Browser](04_stdlib_reference.md#browser) | AI | Headless automation |
| [MCP](04_stdlib_reference.md#mcp) | AI | Model Context Protocol |
| [Agent](04_stdlib_reference.md#agent) | AI | AI agent framework |
| [Graph](04_stdlib_reference.md#graph) | API | GraphQL server |
| [Push](04_stdlib_reference.md#push) | API | Web push notifications |
| [Forms](04_stdlib_reference.md#forms) | API | Form validation |
| [Middleware](04_stdlib_reference.md#middleware) | API | Request middleware |
| [Polyglot](04_stdlib_reference.md#polyglot) | I18n | Internationalization |
| [Lens](04_stdlib_reference.md#lens) | Media | Image processing |
| [Beacon](04_stdlib_reference.md#beacon) | Analytics | Privacy-first tracking |
| [Testing](04_stdlib_reference.md#testing) | Dev | Test utilities |

---

## 🎮 Examples

| Example | Location | Features |
|---------|----------|----------|
| [Recipes](16_templates.md#5-recipes-v3-showcase) | `templates/recipes` | Hello, Counter, Todo, Auth |
| [Shop](16_templates.md#1-shop-state-management) | `templates/shop` | State management |
| [Dashboard](16_templates.md#2-dashboard-analytics) | `templates/dashboard` | CSS, analytics |
| [Chat](16_templates.md#3-chat-real-time) | `templates/chat` | WebSockets |
| Kitchen Sink | `kitchen-sink` | All 19 modules + demos |
| Components Showcase | `projects/site/src/views/components.ncl` | All 7 components |

---

## 📦 Guides by Topic

<details>
<summary><strong>🗄️ Database</strong></summary>

- [Database Guide](#20_database_guide) - CRUD operations
- [Migrations](20_database_guide.md#migrations) - Schema management
- [Relationships](20_database_guide.md#relationships) - One-to-many, many-to-many

</details>

<details>
<summary><strong>🔐 Security</strong></summary>

- [Authentication Guide](#21_authentication_guide)
- [Password Hashing](21_authentication_guide.md#password-hashing)
- [JWT Tokens](21_authentication_guide.md#jwt-tokens-api-authentication)
- [RBAC](21_authentication_guide.md#role-based-access-control-rbac)

</details>

<details>
<summary><strong>🌐 API</strong></summary>

- [API Development](#22_api_development)
- [Error Handling](22_api_development.md#error-handling)
- [Pagination](22_api_development.md#pagination)
- [CORS](22_api_development.md#cors-configuration)

</details>

<details>
<summary><strong>🧩 Components</strong></summary>

- [Components Guide](#26_components_guide)
- [Defining Components](26_components_guide.md#defining-components)
- [Props System](26_components_guide.md#props-system)
- [Slots & Content Projection](26_components_guide.md#slots--content-projection)
- [Scoped CSS](26_components_guide.md#scoped-css)
- [Error Handling](26_components_guide.md#error-handling)

</details>

<details>
<summary><strong>🔗 Integrations</strong></summary>

- [Federation Guide](#25_federation_guide) - Directus, Sanity, Strapi
- [Configuration](#configuration) - .env, TOML, secrets

</details>

<details>
<summary><strong>🚀 Deployment</strong></summary>

- [Deployment & Hosting Guide](#23_deployment_guide)
- [Docker](23_deployment_guide.md#docker-deployment)
- [Cloud Platforms](23_deployment_guide.md#cloud-deployments)
- [Nginx](23_deployment_guide.md#nginx-reverse-proxy)

</details>

---

## 🔧 Operations

*   [Troubleshooting](#08_troubleshooting) - Common errors
*   [Best Practices](#05_best_practices) - Code patterns
*   [Dependency Management](#14_dependency_management) - Vendor modules
*   [Architecture](#13_architecture_improvements) - Internal design
*   [Tooling](#tooling) - CLI, LSP, VS Code
*   [Type Safety](#type_safety) - End-to-end types
*   [Web Standards](#web_standards) - RFC compliance

---

## 🗺️ Roadmap

*   [Roadmap (V3)](#10_roadmap_v3) - Future plans
*   [Gap Analysis](#11_missing_features) - What's coming

---

## 🍳 Recipes

*   [Tailwind CSS](recipes/tailwind_css.md) - CSS framework integration

---

## 📊 Complete Documentation Index

| # | Document | Lines | Topic |
|---|----------|-------|-------|
| 00 | [Master Feature List](#00_master_feature_list) | 100+ | Capabilities |
| 01 | [Getting Started](#01_getting_started) | 300+ | First steps |
| 02 | [Core Concepts](#02_core_concepts) | 50+ | Architecture |
| 03 | [Compiler Reference](#03_compiler_reference) | 30+ | NCC compiler |
| 04 | [Stdlib Reference](#04_stdlib_reference) | 900+ | All 19 modules |
| 05 | [Best Practices](#05_best_practices) | 30+ | Patterns |
| 06 | [Building a CMS](#06_building_a_cms) | 70+ | Tutorial |
| 07 | [Forms & Validation](#07_forms_and_validation) | 35+ | Input handling |
| 08 | [Troubleshooting](#08_troubleshooting) | 35+ | Debug help |
| 09 | [Deployment](#09_deployment) | 30+ | Deploy basics |
| 15 | [State Management](#15_state_management) | 150+ | Signals/stores |
| 16 | [Examples](#16_examples) | 100+ | Code samples |
| 17 | [CLI Reference](#17_cli_reference) | 250+ | All commands |
| 18 | [Complete Reference](#18_complete_reference) | 70+ | Overview |
| 19 | [Syntax Reference](#19_syntax_reference) | 300+ | NCL tags |
| 20 | [Database Guide](#20_database_guide) | 300+ | Photon ORM |
| 21 | [Authentication Guide](#21_authentication_guide) | 350+ | Fortress |
| 22 | [API Development](#22_api_development) | 500+ | REST APIs |
| 23 | [Deployment Guide](#23_deployment_guide) | 280+ | Production |
| 24 | [Quick Start Tutorial](#24_quick_start_tutorial) | 180+ | First app |
| 25 | [Federation Guide](#25_federation_guide) | 200+ | CMS content |
| 25 | [Email Guide](#25_email_guide) | 100+ | SMTP & SES |
| 25 | [Project Structure](#25_project_structure) | 240+ | File layout |
| 26 | [Components Guide](#26_components_guide) | 450+ | UI components |
| 26 | [Image Processing](#26_image_processing_guide) | 130+ | Resize, crop |
| 26 | [Testing Guide](#26_testing_guide) | 270+ | Tests |
| 27 | [Offline Sync](#27_offline_sync_guide) | 170+ | CRDTs |
| 27 | [Social Login](#27_social_login_guide) | 350+ | OAuth |
| 28 | [i18n Guide](#28_i18n_guide) | 150+ | Translations |
| 29 | [Analytics Guide](#29_analytics_guide) | 100+ | Event tracking |
| - | [Configuration](#configuration) | 290+ | Config system |
| - | [Tooling](#tooling) | 55+ | Dev tools |
| - | [Type Safety](#type_safety) | 45+ | Type system |
| - | [Web Standards](#web_standards) | 45+ | Standards |

**Total: 5,000+ lines of documentation**

---

## 🧪 Benchmarks

Run `./benchmarks/run_benchmark.sh` to verify framework speed on your machine.

**Results:** 20,000+ requests/second on M1 MacBook Pro

