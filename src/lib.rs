mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::*;

    // Challenge: Convert hex to base64
    #[test]
    fn test_set1_challenge_1() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let answer = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let base64 = hex_utils::hex_to_base64(hex);
        assert_eq!(base64.unwrap(), answer);
    }

    #[test]
    fn test_hex_to_byte_array() {
        let hex = String::from("0abcdef0");
        let decoded = hex_utils::decode_hex(&hex).unwrap();
        let encoded = hex_utils::encode_hex(&decoded);
        assert_eq!(hex, encoded);
    }
}
