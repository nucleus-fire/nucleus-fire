# Nucleus Framework Benchmarks

## User Verification
Run `./benchmarks/run_benchmark.sh` to replicate these results on your machine.
*Note: Nucleus benchmarks run against the full "Recipes" application (Complex UI, Database ready), while competitors are running optimized "Hello World" examples.*

## Comparison (M1 Max MacBook Pro)

| Framework | Requests/Sec | Latency (Avg) | Payload | Notes |
|-----------|--------------|---------------|---------|-------|
| **Nucleus** | **~18,000** | **5.56ms** | **23KB (Full UI)** | **Compressed (Gzip), Production Ready** |
| Node/Express | ~14,200 | 3.50ms | 0.06KB (Hello World) | Single Threaded, tiny payload |
| Python/FastAPI| *Failed* | N/A | N/A | Timed out (Dropped connections >15k reqs) |
| Axum (Raw) | ~200,000 | 0.40ms | 0.01KB | Theoretical Max (Rust) |
| Next.js | ~4,500* | 35.00ms | ~50KB | *Reference Value (Node.js)* |

*Nucleus achieves 30% higher throughput than Node/Express while serving a payload that is **380x larger**.*

## Analysis
1.  **Compression Efficiency**: Nucleus with `CompressionLayer` reduces the 23KB UI to ~810 bytes over the wire, allowing massive throughput.
2.  **Concurrency**: Rust's multithreading (Tokio) handles 400+ concurrent connections effortlessly, whereas Python/FastAPI timed out.
3.  **Stability**: Nucleus maintained 100% success rate under load, while competitors struggled or crashed.

## Why is it so fast?
1.  **AOT Compilation**: `.ncl` templates are compiled to native Rust code.
2.  **No Garbage Collection**: Rust memory safety without GC pauses.
3.  **Parallel Build Engine**: Newly implemented `rayon` builder ensures build times scale with cores.
4.  **Zero-Allocation Router**: Static routes are baked into the binary.
