// Implementing each set as a library module and each function solution individually

use rustc_serialize::hex::{FromHex, FromHexError, ToHex};
use rustc_serialize::base64::{ToBase64, STANDARD};

pub fn hex_to_base64(input: &str) -> Result<String, FromHexError> {
    match input.from_hex() {
        Ok(hex) => Ok(hex.to_base64(STANDARD)),
        Err(e) => Err(e),
    }
}

pub fn fixed_xor(input_a: &str, input_b: &str) -> Result<String, FromHexError> {
    let a = try!(input_a.from_hex());
    let b = try!(input_b.from_hex());
    let xored_vec: Vec<u8> = a.iter()
        .zip(b.iter())
        .map(|(x, y)| x ^ y)
        .collect();
    Ok(xored_vec.to_hex())
}
