// Implementing each set as a library module and each function solution individually

use rustc_serialize::hex::{FromHex, FromHexError};
use rustc_serialize::base64::{ToBase64, STANDARD};

pub fn hex_to_base64(input: &str) -> Result<String, FromHexError> {
    match input.from_hex() {
        Ok(hex) => Ok(hex.to_base64(STANDARD)),
        Err(e) => Err(e),
    }
}
