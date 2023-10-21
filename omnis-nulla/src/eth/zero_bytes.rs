use anyhow::Result;
use ethers::types::H160;

/// Counts the amount of leading zero bytes in a given Ethereum address.
/// Returns a `u8` indicating the number of leading zero bytes in the address.
pub fn zero_byte_count(eth_address: &H160) -> Result<u8> {
    Ok(eth_address.0.iter().take_while(|&&byte| byte == 0).count() as u8) // Creates an iterator over the inner tuple field holding the byte array of the H160 and continues taking bytes and checking them using a closure as long as they are zero and using count to total the number of zero bytes taken as a u8.
}
