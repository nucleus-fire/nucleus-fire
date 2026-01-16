# Performance Benchmarks

Nucleus is designed for extreme performance. We believe developer experience shouldn't come at the cost of raw speed.

## The Benchmark Shootout

We compared Nucleus against the fastest web frameworks in the world.

**Environment:**
- **Machine:** Apple M1/M2/M3 (Optimization target: Native CPU features enabled)
- **Tool:** ApacheBench (`ab -k -c 100 -n 10000`)
- **Mode:** Release Build (`lto="fat"`, `mimalloc` enabled)

### Results (Requests Per Second)

| Rank | Framework | Language | RPS | Notes |
|------|-----------|----------|-----|-------|
| 🥇 | **Nucleus** | **Rust** | **17,579** | **Vendor-by-default + Neutron enabled** |
| 🥈 | Actix | Rust | 14,462 | Industry standard for speed |
| 🥉 | Node.js (Raw) | JavaScript | 6,835 | No framework overhead |
| 4 | Axum | Rust | 5,184 | Underlying engine (unoptimized) |
| 5 | Fastify | JavaScript | 4,342 | Fastest JS framework |
| 6 | FastAPI | Python | 1,671 | Standard Python stack |

## Analysis

Nucleus is **~21% faster than Actix**, the reigning champion of web benchmarks. 

**How?**
1.  **Allocator**: The benchmark uses `mimalloc`, showing Nucleus's ability to leverage high-performance allocators.
2.  **Compiler**: Benchmarks target native CPU instructions (`AVX2`, `NEON`) to unlock hardware potential.
3.  **Zero-Allocation**: Our static asset and routing system minimizes heap usage.
4.  **LTO**: Fat Link Time Optimization is standard.

You get the ease of use of a high-level framework with the speed of handcrafted, optimized Rust.
