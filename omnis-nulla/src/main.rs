use anyhow::Result;
use ethers::types::H160;
use omnis_nulla_lib::crypto::pk::generate_pk;
use omnis_nulla_lib::eth::address::generate_address;
use omnis_nulla_lib::eth::zero_bytes::zero_byte_count;
use omnis_nulla_lib::setup::setup::initialize;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{error, info};

fn main() -> Result<()> {
    initialize().map_err(|err| {
        error!("Setup initialization failed: {}", err);
        err
    })?;

    info!("Initialization complete");

    let entropy_seed = "xyz";
    let threshold = 1u8;
    let threads: usize = 8;
    let complete = Arc::new(AtomicBool::new(false));

    let product = (0..threads).into_par_iter().find_map_any(|_| {
        let mut hash_iteration = 42u128;
        let mut zero_bytes = 0u8;

        while zero_bytes < threshold && !complete.load(Ordering::Relaxed) {
            hash_iteration += 1;
            info!("Iteration: {:?}", hash_iteration);

            let private_key = match generate_pk(entropy_seed, hash_iteration) {
                Ok(pk) => pk,
                Err(err) => {
                    error!("Failed to assign private key: {}", err);
                    continue;
                }
            };

            info!("Private key retrieved: {:?}", private_key);

            let eth_addr: H160 = match generate_address(&private_key) {
                Ok(addr) => addr,
                Err(err) => {
                    error!("Failed to generate Ethereum address: {}", err);
                    continue;
                }
            };

            info!("Ethereum address generated: {:?}", eth_addr);

            zero_bytes = match zero_byte_count(&eth_addr) {
                Ok(count) => count,
                Err(err) => {
                    error!("Failed to obtain zero byte count: {}", err);
                    continue;
                }
            };

            info!("Zero byte count: {:?}", zero_bytes);

            if zero_bytes >= threshold {
                complete.store(true, Ordering::Relaxed);
                return Some((eth_addr, private_key, hash_iteration));
            }
        }
        None
    });

    match product {
        Some((eth_addr, private_key, hash_iteration)) => {
            info!("FINISHED.\nAddress: {:?}\nPrivate Key: {:?}\nEntropy Seed: {:?}\nHash Iteration: {:?}",
                eth_addr,
                private_key,
                entropy_seed,
                hash_iteration
            );
        }
        None => {
            info!("No valid key found.");
        }
    }

    Ok(())
}
