use anyhow::anyhow;
use sha3::{Digest, Sha3_256};
use std::convert::TryFrom;
use tracing::info;

/// Hashes a given entropy seed along with a counter using SHA3-256 to generate
/// a unique and deterministic 32-byte hash to be used as a private key.
pub fn generate_pk(entropy_seed: &str, hash_iteration: u128) -> anyhow::Result<[u8; 32]> {
    info!("Hashing entropy seed. Iteration: {}", hash_iteration);

    let mut hasher = Sha3_256::new(); // Create a new instance of the SHA3-256 hasher.
    hasher.update(entropy_seed.as_bytes()); // Add entropy seed as bytes to the hasher.
    hasher.update(hash_iteration.to_be_bytes()); // Add the iteration count bytes to the hasher in big-endian format.
    let product = hasher.finalize(); // Finalize the hash computation and store the result.
    let pk = <[u8; 32]>::try_from(product.as_slice())
        .map_err(|e| anyhow!("Failed to convert hash: {}", e))?; // Convert result into 32-byte array, else return error.

    info!(
        "Successfully hashed entropy seed. Iteration: {}",
        hash_iteration
    );
    Ok(pk) // Return 32-byte array wrapped in an `Ok` variant.
}
