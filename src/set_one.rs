// Implementing each set as a library module and each function solution individually

use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::{ToBase64, STANDARD};

#[allow(dead_code)]
pub fn hex_to_base64(input: &str) -> Result<String, &str> {
    let config = STANDARD;
    match input.from_hex() {
        Ok(hex) => Ok(hex.to_base64(config)),
        // TODO don't throw away the Hex error
        Err(_) => Err("Hex conversion failed"),
    }
}
