#![feature(core)]
extern crate "rustc-serialize" as rustc_serialize;

pub mod set_one;

#[cfg(test)]
mod tests {
    use set_one::{hex_to_base64, fixed_xor};

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap().as_slice(),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn test_fixed_xor() {
        assert_eq!(fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965").unwrap().as_slice(),
        "746865206b696420646f6e277420706c6179"
        );
    }
}


