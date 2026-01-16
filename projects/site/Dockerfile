# ═══════════════════════════════════════════════════════════════════════════
# NUCLEUS FRAMEWORK - OPTIMIZED PRODUCTION DOCKERFILE
# Multi-stage build for minimal image size (~20MB)
# ═══════════════════════════════════════════════════════════════════════════

# Stage 1: Build
FROM rust:1.76-buster as builder

WORKDIR /app
COPY . .

# Build release binary with optimizations
RUN cargo build --release --bin server

# Stage 2: Runtime (Distroless for security)
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy only what's needed
COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/static /app/static

# Environment
ENV PORT=3000
EXPOSE 3000

# Run
CMD ["./server"]
