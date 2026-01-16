#!/bin/bash
set -e

echo "🚀 Building Nucleus (Release mode)..."
cargo build --release --bin nucleus

# Start server in background
echo "⚛️  Starting Server..."
./target/release/nucleus run &
SERVER_PID=$!

# Wait for server to start
sleep 2

echo "📊 Running Benchmark (Apache Bench)..."
# 10k requests, 100 concurrency
ab -n 10000 -c 100 http://127.0.0.1:3000/

echo "🛑 Stopping Server..."
kill $SERVER_PID
