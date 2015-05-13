// Implementing each set as a library module and each function solution individually

use rustc_serialize::hex::{FromHex, FromHexError, ToHex};
use rustc_serialize::base64::{ToBase64, STANDARD};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::error::Error;
use collections::string::FromUtf8Error;
use std::fmt;

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
    //let p = Path::new("/home/cr0atian/code/matasano-crypto/resources/4.txt");
    let p = try!(File::open("/home/cr0atian/code/matasano-crypto/resources/4.txt"));
    let mut f = BufReader::new(&p);
    let mut max_score = 0u32;
    let mut current_output = String::new();
    for line in f.lines() {
        let l = line.unwrap();
        let cipher = l.as_str();
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

pub fn repeating_key_xor(key: &'static str, plain: &str) -> String {
    let bkey: &'static [u8] = key.as_bytes();
    let len: usize = key.len();
    let mut rkey = RepeatingKey {key:bkey, curr: bkey[0], curr_index: 0, key_length:len};
    let mut cipher: Vec<u8> = Vec::new();
    for x in plain.as_bytes().iter() {
        cipher.push(x ^ rkey.curr);
        rkey.next();
    }
    cipher.to_hex()
}

struct RepeatingKey {
   key: &'static [u8],
   curr: u8,
   curr_index: usize,
   key_length: usize,
}

impl Iterator for RepeatingKey {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        self.curr_index = if (self.curr_index == (self.key_length - 1)) {0} else {self.curr_index+1};
        self.curr = self.key[self.curr_index];
        Some(self.curr)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct CryptoError {
    pub desc: &'static str,
}

impl Display for CryptoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result {
    }
}


impl Error for CryptoError {
    fn description(&self) -> &str {
        "Error has ocurred"
    }
}


//impl Error<FromUtf8Error> for CryptoError {
    //fn description(&self) -> &str {
        //"Utf 8 Error has ocurred"
    //}
//}
