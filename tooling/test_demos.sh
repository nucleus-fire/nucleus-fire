#!/bin/bash
set -e

echo "🧪 Starting End-to-End Demo Verification..."

# 1. Dashboard
echo "📊 Verifying Dashboard..."
cd examples/dashboard
cargo check
cargo test
cd ../..

# 2. Chat
echo "💬 Verifying Chat..."
cd examples/chat
cargo check
cargo test
cd ../..

# 3. Shop
echo "🛒 Verifying Shop..."
cd examples/shop
cargo check
cargo test
cd ../..

# 4. Docs Site
echo "📚 Verifying Docs Site..."
# Run from root so that CWD is correct for 'docs/' access
cargo check --manifest-path docs-site/Cargo.toml
cargo test --manifest-path docs-site/Cargo.toml


echo "✅ All Demos Verified Successfully!"
