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

pub fn single_byte_xor(input: &str) -> Result<String, FromHexError> {
    let cipher = try!(input.from_hex());
    // iterating through all of ASCII
    let mut answer: Vec<u8> = Vec::new();
    let mut best_score = 0u32;
    for xor_byte in (0u8..255) {
        let mut plain: Vec<u8> = Vec::new();
        let mut score = 0u32;
        for i in cipher.iter() {
            let character = i ^ xor_byte;
            score += score_character(character as char);
            plain.push(character);
        }
        if score > best_score {
            best_score = score;
            answer = plain;
        }
    }
    Ok(String::from_utf8(answer).unwrap())
}

fn score_character(x: char) -> u32 {
    if x == 'e' || x == 't' || x == 'a' {
        4
    } else if x == 'o' || x == 'i' || x == 'n' {
        3
    } else if x == 's' || x == 'h' || x == 'r' {
        2
    } else if x == 'd' || x == 'l' || x == 'u' {
        1
    } else {
        0
    }
}
