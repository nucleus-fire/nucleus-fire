# Crypto & Chain Guide

Interact with blockchain networks and verify identities using Nucleus Chain.

## Overview

The `Chain` module provides utilities for:
- **EIP-191 Signature Verification**: Securely verify "Login with Ethereum" signatures.
- **Balance Checks**: Query native token balances (ETH, MATIC, etc.) via RPC.
- **Address Validation**: Check Ethereum-compatible addresses.

## Quick Start

```rust
use nucleus_std::chain::Chain;

// 1. Verify a signature
let message = "Login to Nucleus App directly";
let signature = "0x..."; // 65-byte hex signature
let address = "0x71C7656EC7ab88b098defB751B7401B5f6d8976F";

let is_valid = Chain::verify_signature(message, signature, address)?;

if is_valid {
    println!("User authenticated!");
}

// 2. Get Balance (Requires RPC config)
let balance = Chain::get_native_balance(address).await?;
println!("Balance: {} ETH", balance);
```

## Configuration

To use RPC features (like `get_native_balance`), configure your RPC URL in `nucleus.config` or environment variables:

```toml
[chain]
rpc_url = "https://mainnet.infura.io/v3/YOUR_KEY"
```

Or via environment:

```bash
CHAIN_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY
```

## API Reference

### `Chain::verify_signature`

Verifies a standard EIP-191 "Ethereum Signed Message".

```rust
pub fn verify_signature(
    message: &str, 
    signature: &str, 
    address: &str
) -> Result<bool>
```

- **message**: The raw text message the user signed. The function automatically applies the EIP-191 prefix (`\x19Ethereum Signed Message:\n` + length).
- **signature**: The hex-encoded signature (starting with `0x`).
- **address**: The implementation checks if the recovered public key matches this address.

### `Chain::get_native_balance`

Fetches the native token balance for an address.

```rust
pub async fn get_native_balance(address: &str) -> Result<Decimal>
```

- Returns a `Decimal` representing the balance in Ether/Unit (automatically converts from Wei).
- Requires `rpc_url` to be configured.

## Example: Login with Ethereum

Implementation pattern for a login handler:

```rust
use nucleus_std::chain::Chain;

#[derive(Deserialize)]
struct LoginRequest {
    address: String,
    signature: String,
    nonce: String, // Message to sign
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, AppError> {
    // 1. Verify signature
    let is_valid = Chain::verify_signature(
        &payload.nonce,
        &payload.signature,
        &payload.address
    )?;

    if !is_valid {
        return Err(AppError::InvalidSignature);
    }

    // 2. Find or Create User
    let user = User::find_by_wallet(&payload.address).await?;

    // 3. Issue Token
    let token = fortress::create_token(&user)?;

    Ok(Json(AuthResponse { token }))
}
```
