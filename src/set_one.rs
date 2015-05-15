// Implementing each set as a library module and each function solution individually

use rustc_serialize::hex::{FromHex, FromHexError, ToHex};
use rustc_serialize::base64::{ToBase64, STANDARD};
use std::io::{BufReader, BufRead};
use std::io;
use std::fs::File;
use std::error::Error;
use std::string::{String, FromUtf8Error};
use std::fmt;
use num::traits::PrimInt;

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
    let (_, answer) = score_string(input).unwrap();
    Ok(answer)
}

pub fn detect_single_byte_xor() -> Result<String, io::Error> {
    //let p = Path::new("/home/cr0atian/code/matasano-crypto/resources/4.txt");
    let p = try!(File::open("resources/4.txt"));
    let f = BufReader::new(&p);
    let mut max_score = 0u32;
    let mut current_output = String::new();
    for line in f.lines() {
        let l = line.unwrap();
        let cipher = &l;
        let (score, output) = score_string(cipher).unwrap();
        if score > max_score {
            max_score = score;
            current_output = output;
        }
        println!("{} : {}", score, current_output);
    }
    Ok(current_output)
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
        Err(_) => Ok((best_score, "".to_string()))
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

pub fn hamming_distance(a: &str, b: &str) -> u32 {
    a.as_bytes().iter()
        .zip(b.as_bytes().iter())
        .fold(0, |acc, (&x, &y)| acc + (x^y).count_ones())
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
        self.curr_index = if self.curr_index == (self.key_length - 1) {0} else {self.curr_index+1};
        self.curr = self.key[self.curr_index];
        Some(self.curr)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CryptoError {
    pub desc: &'static str,
}


impl From<FromUtf8Error> for CryptoError {
    fn from(_: FromUtf8Error) -> Self {
       CryptoError {
           desc: "no valid Utf-8 decoded",
       }
    }
}
impl From<FromHexError> for CryptoError {
    fn from(_: FromHexError) -> Self {
       CryptoError {
           desc: "invalid hex input",
       }
    }
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{}", self.desc)
    }
}

