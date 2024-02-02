use rand::{thread_rng, Rng};
extern crate base64;
extern crate sha2;

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use sha2::{Digest, Sha512};

/// Generates hash value for the given payload using sha512 algorithm
///
/// References:
///     https://docs.rs/sha2/latest/sha2/#usage
pub fn calculate_hash(value: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(value);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Creates a Base64-encoded ASCII string from a binary string (similar to the built-in btoa function in native JS)
///
/// (i.e., a string in which each character in the string is treated as a byte of binary data)
///
/// References:
///     https://docs.rs/base64/latest/base64/#url-safe-alphabet
#[allow(dead_code)]  // Just for reference
pub fn base64_encode(value: &str) -> String {
    URL_SAFE.encode(value.as_bytes())
}

/// Decode a string of data which has been encoded using base64 (similar to the built-in atob function in native JS)
///
/// References:
///     https://docs.rs/base64/latest/base64/#url-safe-alphabet
pub fn base64_decode(value: &str) -> String {
    if let Ok(decoded_bytes) = URL_SAFE.decode(value) {
        if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
            decoded_str
        } else {
            panic!("Error decoding UTF-8");
        }
    } else {
        panic!("Error decoding base64");
    }
}

/// Convert a string value into hex
pub fn hex_encode(value: &str) -> String {
    let mut hex_values: Vec<String> = Vec::new();
    for ch in value.chars() {
        let hex_value = format!("{:04x}", ch as u32);
        hex_values.push(hex_value);
    }
    format!("\\u{}", hex_values.join("\\u"))
}

/// Convert hex value into a string
pub fn hex_decode(value: &str) -> String {
    let mut result = String::new();
    let hex_values: Vec<&str> = value.split("\\u").skip(1).collect();
    for hex_value in hex_values {
        if let Ok(code_point) = u32::from_str_radix(hex_value, 16) {
            if let Some(ch) = char::from_u32(code_point) {
                result.push(ch);
            }
        }
    }
    result
}

pub fn keygen() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut rng = thread_rng();
    let token: String = (0..64)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
   token
}
