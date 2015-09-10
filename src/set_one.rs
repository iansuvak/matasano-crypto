// Implementing each set as a library module and each function solution individually

use rustc_serialize::hex::{FromHex, FromHexError, ToHex};
use rustc_serialize::base64::{ToBase64, STANDARD, FromBase64, FromBase64Error};
use std::io::{BufReader, BufRead, Read};
use std::io;
use std::fs::File;
use std::error::Error;
use std::string::{String, FromUtf8Error};
use std::fmt;
use num::traits::PrimInt;
use std::str::from_utf8;

const MIN_KEY_LENGTH: usize = 2;
const MAX_KEY_LENGTH: usize = 40;
const MAX_SCORE: f32 = MAX_KEY_LENGTH as f32 * 8.0;

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
    let (_, answer, _) = score_hex_string(input).unwrap();
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
        let (score, output, _) = score_hex_string(cipher).unwrap();
        if score > max_score {
            max_score = score;
            current_output = output;
        }
    }
    Ok(current_output)
}

fn score_hex_string(input: &str) -> Result<(u32, String, u8), CryptoError> {
    score_string(try!(input.from_hex()))
}

fn score_string(cipher: Vec<u8>) -> Result<(u32, String, u8), CryptoError> {
    // iterating through all of ASCII
    let mut answer: Vec<u8> = Vec::new();
    let mut best_score = 0u32;
    let mut key_byte = 0u8;
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
            key_byte = xor_byte;
        }
    }
    let result = String::from_utf8(answer);
    match result {
        Ok(output) => Ok((best_score, output, key_byte)),
        Err(_) => Ok((best_score, "".to_string(), key_byte))
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

pub fn repeating_key_xor(key: &str, plain: &str) -> String {
    let bkey: &[u8] = key.as_bytes();
    let len: usize = key.len();
    let mut rkey = RepeatingKey {key:bkey, curr: bkey[0], curr_index: 0, key_length:len};
    let mut cipher: Vec<u8> = Vec::new();
    for x in plain.as_bytes().iter() {
        cipher.push(x ^ rkey.curr);
        rkey.next();
    }
    cipher.to_hex()
}

pub fn hamming_distance(a: &str, b: &str) -> Result<u32, CryptoError>  {
    if a.len() != b.len() {
        Err(CryptoError { desc: "Cannot calculate hamming distance of strings with unequal lengths"})
    } else {
        Ok(a.as_bytes().iter()
            .zip(b.as_bytes().iter())
            .fold(0, |acc, (&x, &y)| acc + (x^y).count_ones()))
    }
}

pub fn vigenere(path: &str) -> Result<String, CryptoError> {
    let mut cipher : Vec<u8> = Vec::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_end(&mut cipher));
    cipher = cipher.from_base64().unwrap();
    let cipher_length = cipher.len();
    print!("Cipher Length is {}", cipher_length);
    let max_key = if cipher_length / 8 < MAX_KEY_LENGTH {
            cipher_length / 8
        } else {
            MAX_KEY_LENGTH
        };
    let cipher_slice: &[u8] = &cipher;
    let mut current_min_score = MAX_SCORE;
    let mut current_length_guess = 0;
    for key_length in MIN_KEY_LENGTH..max_key {
        let mut blocks: Vec<&str> = Vec::new();
        for block in cipher_slice.chunks(key_length).take(8) {
            blocks.push(from_utf8(block).unwrap());
        }
        let new_score: f32 = try!(average_distance(blocks, key_length));
        if new_score < current_min_score {
            current_min_score = new_score;
            current_length_guess = key_length;
        }
    }
    // Now we know the key_length
    print!("Min score is {}, keylength is {}\n", current_min_score, current_length_guess);
    let keylength = current_length_guess;
    // collation code to split the vector into blocks
    let mut full_key = Vec::new();
    for offset in 0..keylength {
        let mut collated_block = Vec::new();
        for block in 0..(cipher_slice.len() / keylength) {
            collated_block.push(cipher_slice[block * keylength + offset]);
        }
        // calculate key byte for the current offset in keylength
        let (_, _, key_char) = score_string(collated_block).unwrap();
        full_key.push(key_char);
    }
    print!("Full key is \"{}\"\n", from_utf8(&full_key).unwrap());
    let decoded = try!((repeating_key_xor(from_utf8(&full_key).unwrap(), from_utf8(&cipher_slice).unwrap())).from_hex());
    print!("decoded string is {}", from_utf8(&decoded).unwrap());
    Ok("result".to_string())
}

// TODO replace with permutations once stabilized
fn average_distance(blocks: Vec<&str>, key_length: usize) -> Result<f32, CryptoError> {
    let mut score = 0f32;
    for i in 0..6 {
        for j in i+1..7 {
            score += try!(hamming_distance(blocks[i], blocks[j])) as f32;
        }
    }
    // Score will scale equally since we are doing the same number of comparisons
    Ok(score / key_length as f32)
}

struct RepeatingKey<'a> {
   key: &'a [u8],
   curr: u8,
   curr_index: usize,
   key_length: usize,
}

impl<'a> Iterator for RepeatingKey<'a> {
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

impl From<FromBase64Error> for CryptoError {
    fn from(_: FromBase64Error) -> Self {
       CryptoError {
           desc: "invalid base64 input",
       }
    }
}
impl From<io::Error> for CryptoError {
    fn from(_: io::Error) -> Self {
       CryptoError {
           desc: "Failed to open file",
       }
    }
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

