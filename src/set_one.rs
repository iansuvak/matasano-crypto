// Implementing each set as a library module and each function solution individually

use rustc_serialize::hex::{FromHex, FromHexError, ToHex};
use rustc_serialize::base64::{ToBase64, STANDARD};
use std::old_path::Path;
use std::old_io::{BufferedReader, File};
use std::error::FromError;
use collections::string::FromUtf8Error;

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
    let (score, answer) = score_string(input).unwrap();
    Ok(answer)
}

pub fn detect_single_byte_xor() -> String {
    let p = Path::new("/home/cr0atian/code/matasano-crypto/resources/4.txt");
    let mut f = BufferedReader::new(File::open(&p));
    let mut max_score = 0u32;
    let mut current_output = String::new();
    for line in f.lines() {
        let l = line.unwrap();
        let cipher = l.as_slice();
        let (score, output) = score_string(cipher).unwrap();
        if score > max_score {
            max_score = score;
            current_output = output;
        }
        println!("{} : {}", score, current_output);
    }
    current_output
}

fn score_string(input: &str) -> Result<(u32, String), CryptoError> {
    let cipher = try!(input.from_hex());
    // iterating through all of ASCII
    let mut answer: Vec<u8> = Vec::new();
    let mut best_score = 0u32;
    for xor_byte in (32u8..127) {
        let mut plain: Vec<u8> = Vec::new();
        let mut score = 0u32;
        for i in cipher.iter() {
            let character = i ^ xor_byte;
            //if character < 32u8 || character > 127u8 {
                //return Err(CryptoError { desc: "Xored string out of utf-8 range", })
            //}
            score += score_character(character as char);
            plain.push(character);
        }
        if score > best_score {
            best_score = score;
            answer = plain;
        }
    }
    let result = String::from_utf8(answer);
    match result {
        Ok(output) => Ok((best_score, output)),
        Err(_) => Ok((best_score, String::from_str("")))
    }
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CryptoError {
    pub desc: &'static str,
}

impl FromError<FromHexError> for CryptoError {
    fn from_error(err: FromHexError) -> CryptoError {
        CryptoError {
            desc: "Hex Error has ocurred",
        }
    }
}

impl FromError<FromUtf8Error> for CryptoError {
    fn from_error(err: FromUtf8Error) -> CryptoError {
        CryptoError {
            desc: "Utf 8 Error has ocurred",
        }
    }
}
