use anyhow::{anyhow, Result};
use ethers::types::H160;
use libsecp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use tracing::info;

/// Derives an Ethereum address from a given private key.
/// Returns a result containing either `Ok(())` if successful, or an error detailing the issue.
pub fn generate_address(private_key: &[u8; 32]) -> Result<H160> {
    info!("Generating ethereum address using: {:?}", private_key);

    let wrapped_private_key = SecretKey::parse_slice(private_key) // Parse private_key into SecretKey format to enable compatibility with the secp256k1 curve.
        .map_err(|e| anyhow!("Failed to parse private key: {}", e))?;

    let pub_key = PublicKey::from_secret_key(&wrapped_private_key).serialize(); // Derive and serialize public key into byte format.

    let mut hashing_algo = Keccak::v256(); // Declare hashing algo.
    let mut hash = [0u8; 32]; // Init 32 byte (256 bit) array with 8-bit uint 0 values to store the hashed product.

    hashing_algo.update(&pub_key[1..]); // Update hash state with bytes of serialized pub_key excluding the first byte.
    hashing_algo.finalize(&mut hash); // Compute final hash and write result into the `hash` array.

    let mut eth_address_buffer = H160::default();
    eth_address_buffer.assign_from_slice(&hash[12..]); // Extract and assign last 20 bytes of hash.

    Ok(eth_address_buffer)
}
