#!/bin/bash
echo "🚀 Starting Server..."
./target/release/server > server.log 2>&1 &
SERVER_PID=$!
sleep 2

echo "🔍 Curling with gzip..."
curl -v -H "Accept-Encoding: gzip" http://127.0.0.1:3002/ > curl_output.txt 2>&1

echo "🛑 Stopping Server..."
kill $SERVER_PID

cat curl_output.txt
