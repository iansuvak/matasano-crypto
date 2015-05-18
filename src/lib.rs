extern crate rustc_serialize;
extern crate num;

pub mod set_one;

#[cfg(test)]
mod tests {
    use set_one::{hex_to_base64, fixed_xor, single_byte_xor, detect_single_byte_xor, repeating_key_xor, hamming_distance};

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(&hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap(),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn test_fixed_xor() {
        assert_eq!(&fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965").unwrap(),
        "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    fn test_single_byte_xor() {
        assert_eq!(&single_byte_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap(),
        "Cooking MC\'s like a pound of bacon"
        );
    }

    #[test]
    fn test_detect_single_byte_xor() {
        assert_eq!(&detect_single_byte_xor().unwrap(),
        "Now that the party is jumping\n"
        );
    }

    #[test]
    fn test_repeating_key_xor() {
        assert_eq!(&repeating_key_xor("ICE", "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"),
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!").unwrap(), 37);
    }
}


