#!/bin/bash
set -e

# Configuration
URL="http://127.0.0.1:3000/"
DURATION="10s"
THREADS="12"
CONNECTIONS="400"

echo "🚀 Nucleus Stress Test Engine"
echo "============================="

# 1. Build Recipes App (Release Mode)
echo "🔨 Building examples/recipes (Release)..."
cargo build --release --bin server --manifest-path examples/recipes/Cargo.toml

# 2. Start Server
SERVER_BIN="target/release/server"
echo "⚛️  Starting Server..."
$SERVER_BIN > /dev/null 2>&1 &
SERVER_PID=$!
sleep 0.5 # Initial startup buffer

# Wait for port 3002 to be open
echo "⏳ Waiting for port 3002..."
for i in {1..20}; do
    if nc -z 127.0.0.1 3002 2>/dev/null; then
        echo "✅ Server is up!"
        break
    fi
    sleep 0.5
done
sleep 1 # Extra buffer after port open

# 3. Running Stress Test
echo "🔍 verifying compression..."
curl -I -H "Accept-Encoding: gzip" $URL

echo "🔥 Attacking $URL with $CONNECTIONS connections for $DURATION..."

if command -v wrk &> /dev/null; then
    wrk -t$THREADS -c$CONNECTIONS -d$DURATION -H "Accept-Encoding: gzip, deflate, br" $URL
elif command -v ab &> /dev/null; then
    echo "⚠️  'wrk' not found, falling back to Apache Bench (slower client)."
    ab -n 10000 -c 100 -H "Accept-Encoding: gzip, deflate, br" $URL
else
    echo "❌ No stress test tool found. Please install 'wrk' (brew install wrk)."
    kill $SERVER_PID
    exit 1
fi

# 4. Cleanup
echo "🛑 Stopping Server (PID: $SERVER_PID)..."
kill $SERVER_PID
echo "✅ Done."
