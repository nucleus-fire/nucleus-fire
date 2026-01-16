#!/bin/bash
set -e

RESULTS_FILE="results.txt"
echo "Framework Shootout Results" > $RESULTS_FILE
echo "==========================" >> $RESULTS_FILE

function benchmark {
    NAME=$1
    PORT=$2
    CMD=$3
    WD=$4
    
    echo "------------------------------------------------"
    echo "🏎️  Preparing $NAME..."
    
    cd $WD
    # Build
    if [ "$NAME" == "Nucleus" ]; then
        RUSTFLAGS="-C target-cpu=native" cargo build --release --bin server
        SERVER_CMD="../../target/release/server"
    elif [[ "$NAME" == *"Node"* ]] || [[ "$NAME" == "Fastify" ]]; then
        echo "Skipping Cargo build for Node/Fastify..."
        SERVER_CMD="node server.js"
    elif [[ "$NAME" == "FastAPI" ]]; then
        echo "Skipping Cargo build for FastAPI..."
        SERVER_CMD="venv/bin/python main.py"
    elif [[ "$NAME" == "Next.js" ]] || [[ "$NAME" == "Remix" ]]; then
        echo "Building $NAME..."
        # Ensure deps are installed if node_modules missing (safety check)
        if [ ! -d "node_modules" ]; then
             npm install
        fi
        npm run build
        SERVER_CMD="npm start"
    else
        RUSTFLAGS="-C target-cpu=native" cargo build --release
        SERVER_CMD="../../target/release/$NAME"
    fi
    
    # Run
    echo "🟢 Starting $NAME on port $PORT..."
    env PORT=$PORT $SERVER_CMD &
    PID=$!
    # Give JS frameworks a bit more time to boot
    sleep 8
    
    # Bench
    echo "📊 Benchmarking $NAME..."
    # Warmup
    ab -n 1000 -c 10 http://127.0.0.1:$PORT/ > /dev/null 2>&1
    # Real Run
    OUTPUT=$(ab -n 10000 -c 100 http://127.0.0.1:$PORT/ | grep "Requests per second")
    RPS=$(echo $OUTPUT | awk '{print $4}')
    
    echo "$NAME: $RPS RPS"
    echo "$NAME: $RPS RPS" >> ../$RESULTS_FILE
    
    kill $PID || true
    cd ..
    sleep 2
}

# 1. Actix (Enabled)
# benchmark "actix_bench" 3001 "cargo run --release" "actix_bench"

# 2. Axum
# benchmark "axum_bench" 3002 "cargo run --release" "axum_bench"

# 3. Node.js (Raw HTTP)
# benchmark "Node (Raw)" 3003 "node server.js" "node_bench"

# 4. Fastify (Node Framework)
# benchmark "Fastify" 3004 "node server.js" "fastify_bench"

# 5. FastAPI (Python)
# benchmark "FastAPI" 3005 "venv/bin/python main.py" "fastapi_bench"

# 6. Nucleus
benchmark "Nucleus" 3000 "cargo build --release --bin server" "nucleus_bench"

# 7. Next.js
# benchmark "Next.js" 3006 "npm start" "next_bench"

# 8. Remix
# benchmark "Remix" 3007 "npm start" "remix_bench"

cat $RESULTS_FILE
