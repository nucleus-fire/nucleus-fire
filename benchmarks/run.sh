#!/bin/bash

# Configuration
DURATION="10s"
CONNECTIONS=50
THREADS=4 # wrk supports threads, ab does not. We'll use ab as fallback.
URL="http://127.0.0.1"

# Colors
GREEN='\033[0;32m'
NC='\033[0m' # No Color

function run_benchmark {
    NAME=$1
    PORT=$2
    CMD_START=$3
    
    echo -e "\n${GREEN}=== Benchmarking $NAME on port $PORT ===${NC}"
    echo "Starting server: $CMD_START"
    
    # Start Server in Background
    eval "$CMD_START" &
    PID=$!
    
    # Wait for startup
    sleep 3
    
    # Run Benchmark (using ab because wrk is missing)
    # 50,000 requests to get a good average
    ab -n 50000 -c $CONNECTIONS -k "$URL:$PORT/" > "benchmarks/${NAME}_result.txt" 2>&1
    
    # Parse Result
    RPS=$(grep "Requests per second" "benchmarks/${NAME}_result.txt" | awk '{print $4}')
    LATENCY=$(grep "Time per request:.*(mean)" "benchmarks/${NAME}_result.txt" | head -n 1 | awk '{print $4}')
    
    echo -e "${GREEN}Result: $RPS req/sec | $LATENCY ms latency${NC}"
    
    # Kill Server
    kill $PID
    pkill -P $PID # Kill children if any
    wait $PID 2>/dev/null
    sleep 2
}

# Ensure ports are clear
echo "Cleaning ports..."
lsof -i :3000 -t | xargs kill -9 2>/dev/null
lsof -i :3001 -t | xargs kill -9 2>/dev/null
lsof -i :3002 -t | xargs kill -9 2>/dev/null

# 1. Nucleus AOT
# Note: We use the already built binary from zenith-showcase
run_benchmark "Nucleus_AOT" 3000 "./zenith-showcase/target/release/server"

# 2. Node.js Express
run_benchmark "Node_Express" 3001 "node benchmarks/node/server.js"

# 3. Python FastAPI
# Using venv uvicorn
# Uvicorn needs to be run as module or script. 
# "uvicorn main:app --port 3002 --log-level error"
run_benchmark "Python_FastAPI" 3002 "cd benchmarks/python && ./venv/bin/uvicorn main:app --port 3002 --log-level error"

echo -e "\n${GREEN}=== Benchmark Complete ===${NC}"
cat benchmarks/*_result.txt | grep "Requests per second"
