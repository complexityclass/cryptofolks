use std::fmt::Write;
use std::num::ParseIntError;
extern crate base64;

pub fn hex_to_base64(hex: &str) -> Option<String> {
    let bhex = decode_hex(hex).ok()?;
    Some(base64::encode(&bhex))
}

pub fn decode_hex(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    let s = if hex.len() % 2 == 0 {
        format!("{}", hex)
    } else {
        format!("0{}", hex)
    };
    decode_hex_strict(&s)
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

fn decode_hex_strict(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}
