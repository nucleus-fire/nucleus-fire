use crate::config::GLOBAL_CONFIG;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use rust_decimal::Decimal;
use serde_json::json;
use std::str::FromStr;
use sha3::{Digest, Keccak256};

pub struct Chain;

use crate::errors::{Result, NucleusError};
use serde::de::Error; // For custom TOML errors

impl Chain {
    /// Verify an EIP-191 Ethereum Signature
    /// Message format: "\x19Ethereum Signed Message:\n" + length + message
    pub fn verify_signature(message: &str, signature: &str, address: &str) -> Result<bool> {
        let signature = signature.strip_prefix("0x").unwrap_or(signature);
        if signature.len() != 130 {
            return Err(NucleusError::CryptoError("Invalid signature length: expected 130 hex chars (65 bytes)".to_string()));
        }

        let r_hex = &signature[0..64];
        let s_hex = &signature[64..128];
        let v_hex = &signature[128..130];

        let r_bytes = hex::decode(r_hex).map_err(|e| NucleusError::CryptoError(format!("Invalid R hex: {}", e)))?;
        let s_bytes = hex::decode(s_hex).map_err(|e| NucleusError::CryptoError(format!("Invalid S hex: {}", e)))?;
        let v_byte = hex::decode(v_hex).map_err(|e| NucleusError::CryptoError(format!("Invalid V hex: {}", e)))?[0];

        // Recovery ID adjustment (27/28 -> 0/1)
        let rec_id = if v_byte >= 27 { v_byte - 27 } else { v_byte };
        let recovery_id = RecoveryId::from_byte(rec_id).ok_or(NucleusError::CryptoError("Invalid recovery ID".to_string()))?;

        let signature_bytes = [r_bytes.as_slice(), s_bytes.as_slice()].concat();
        let signature_obj = Signature::from_slice(&signature_bytes).map_err(|e| NucleusError::CryptoError(format!("Invalid signature bytes: {}", e)))?;

        // EIP-191 Prefixing
        let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
        let full_message = [prefix.as_bytes(), message.as_bytes()].concat();
        
        // Keccak256 Hash of the prefixed message
        let mut hasher = Keccak256::new();
        hasher.update(&full_message);
        let digest = hasher.finalize();

        // Recover Public Key
        let verifying_key = VerifyingKey::recover_from_prehash(&digest, &signature_obj, recovery_id)
            .map_err(|_| NucleusError::CryptoError("Failed to recover public key from signature".to_string()))?;

        // Public Key -> Ethereum Address
        // Uncompressed pubkey has 65 bytes, first is 0x04
        let encoded_point = verifying_key.to_encoded_point(false);
        let pub_key_bytes = encoded_point.as_bytes();
        // Skip first byte (0x04) for uncompressed key
        let mut addr_hasher = Keccak256::new();
        addr_hasher.update(&pub_key_bytes[1..]);
        let addr_digest = addr_hasher.finalize();

        let recovered_address = hex::encode(&addr_digest[12..]);
        
        let target_address = address.strip_prefix("0x").unwrap_or(address).to_lowercase();
        
        Ok(recovered_address == target_address)
    }

    /// Get Native Balance (ETH/MATIC/etc) via RPC
    pub async fn get_native_balance(address: &str) -> Result<Decimal> {
        let config = GLOBAL_CONFIG.chain.as_ref().ok_or(NucleusError::ConfigError(toml::de::Error::custom("Chain config not set in nucleus.config")))?;
        
        let client = reqwest::Client::new();
        let payload = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [address, "latest"],
            "id": 1
        });

        let res = client.post(&config.rpc_url)
            .json(&payload)
            .send()
            .await?; 
            // reqwest::Error automatically mapped to NucleusError::NetworkError

        let json: serde_json::Value = res.json().await?;
        
        if let Some(hex_bal) = json["result"].as_str() {
            let hex_clean = hex_bal.strip_prefix("0x").unwrap_or(hex_bal);
            let wei = u128::from_str_radix(hex_clean, 16).map_err(|e| NucleusError::CryptoError(format!("Invalid hex balance: {}", e)))?;
            
            // Convert WEI to Ether (1e18)
            let bal = Decimal::from_str(&wei.to_string()).unwrap() / Decimal::from_str("1000000000000000000").unwrap();
            Ok(bal)
        } else {
            Err(NucleusError::CryptoError(format!("RPC Response Error: {:?}", json)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_signature_valid() {
        use k256::ecdsa::SigningKey;
        // use k256::elliptic_curve::sec1::ToEncodedPoint; // Apparently unused or available in prelude
        use rand::rngs::OsRng;

        // 1. Generate Keypair
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        
        // 2. Derive Address from Public Key
        let address = get_address(verifying_key);

        // 3. Prepare Message & Hash (EIP-191)
        let message = "Hello Nucleus!";
        let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
        let full_message = [prefix.as_bytes(), message.as_bytes()].concat();
        
        let mut hasher = Keccak256::new();
        hasher.update(&full_message);
        let digest = hasher.finalize();

        // 4. Sign Hash (Recoverable)
        let (signature, rec_id) = signing_key.sign_prehash_recoverable(&digest).expect("Failed to sign");
        
        // 5. Construct 65-byte Signature (R + S + V)
        let r_bytes = signature.r().to_bytes();
        let s_bytes = signature.s().to_bytes();
        let v = rec_id.to_byte() + 27; // EIP-191 standard V
        
        let mut sig_bytes = Vec::new();
        sig_bytes.extend_from_slice(&r_bytes);
        sig_bytes.extend_from_slice(&s_bytes);
        sig_bytes.push(v);
        
        let signature_hex = hex::encode(sig_bytes);
        
        // 6. Verify using our Chain module
        let result = Chain::verify_signature(message, &signature_hex, &address);
        assert!(result.is_ok(), "Verification failed");
        assert!(result.unwrap(), "Signature verification failed for valid signature");
    }
    
    fn get_address(key: &VerifyingKey) -> String {
        let encoded = key.to_encoded_point(false);
        let bytes = encoded.as_bytes();
        let mut hasher = Keccak256::new();
        hasher.update(&bytes[1..]);
        let digest = hasher.finalize();
        hex::encode(&digest[12..])
    }
    
    #[test]
    fn test_verify_signature_invalid() {
        use k256::ecdsa::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let address = get_address(verifying_key);
        
        let message = "Test Message";
        
        // Case A: Totally invalid signature string
        let signature_hex = "00".repeat(65);
        let result = Chain::verify_signature(message, &signature_hex, &address);
        // It's acceptable for this to be Err or Ok(false).
        if let Ok(valid) = result {
             assert!(!valid, "Should be false for invalid sig");
        } // Error is also acceptable (parsing failure)
        
        // Case B: Valid Signature for Different Message
        // Sign "Other Message"
        let msg2 = "Other Message";
        let prefix = format!("\x19Ethereum Signed Message:\n{}", msg2.len());
        let full_message = [prefix.as_bytes(), msg2.as_bytes()].concat();
        let mut hasher = Keccak256::new();
        hasher.update(&full_message);
        let digest = hasher.finalize();
        
        let (sig, recid) = signing_key.sign_prehash_recoverable(&digest).unwrap();
        let v = recid.to_byte() + 27;
        
        let mut sig_bytes = Vec::new();
        sig_bytes.extend_from_slice(&sig.r().to_bytes());
        sig_bytes.extend_from_slice(&sig.s().to_bytes());
        sig_bytes.push(v);
        
        let wrong_msg_sig = hex::encode(sig_bytes);

        // Verify "Test Message" with signature of "Other Message"
        let result = Chain::verify_signature(message, &wrong_msg_sig, &address);
        assert!(matches!(result, Ok(false)), "Should return Ok(false) for mismatched signature");
    }
}
