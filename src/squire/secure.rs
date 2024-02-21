extern crate base64;
extern crate sha2;

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rand::{Rng, thread_rng};
use sha2::{Digest, Sha512};

/// Calculates the SHA-512 hash value for the given payload.
///
/// # Arguments
///
/// * `value` - The payload for which the hash is to be calculated.
///
/// # Returns
///
/// A hexadecimal string representing the SHA-512 hash value.
///
/// ## References
/// - [Official docs](https://docs.rs/sha2/latest/sha2/#usage)
pub fn calculate_hash(value: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(value);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Encodes a binary string to a Base64-encoded ASCII string.
///
/// This function is similar to the built-in `btoa` function in native JavaScript,
/// treating each character in the input string as a byte of binary data.
///
/// # Arguments
///
/// * `value` - The binary string to be encoded.
///
/// # Returns
///
/// A Base64-encoded ASCII string.
///
/// ## References
/// - [Official docs](https://docs.rs/base64/latest/base64/#url-safe-alphabet)
#[allow(dead_code)]  // Just for reference
pub fn base64_encode(value: &str) -> String {
    URL_SAFE.encode(value.as_bytes())
}

/// Decodes a Base64-encoded string to its original binary representation.
///
/// This function is similar to the built-in `atob` function in native JavaScript.
///
/// # Arguments
///
/// * `value` - The Base64-encoded string to be decoded.
///
/// # Returns
///
/// A `Result` containing the decoded string or an error message.
///
/// ## References
/// - [Official Docs](https://docs.rs/base64/latest/base64/#url-safe-alphabet)
pub fn base64_decode(value: &str) -> Result<String, &'static str> {
    if let Ok(decoded_bytes) = URL_SAFE.decode(value) {
        if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
            return Ok(decoded_str);
        } else {
            log::error!("Error decoding UTF-8");
        }
    } else {
        log::error!("Error decoding Base64");
    }
    Err("Server was unable to decrypt the credentials")
}

/// Encodes a string value into a hexadecimal representation.
///
/// # Arguments
///
/// * `value` - The string value to be encoded.
///
/// # Returns
///
/// A string representing the hexadecimal encoding of the input.
pub fn hex_encode(value: &str) -> String {
    let mut hex_values: Vec<String> = Vec::new();
    for ch in value.chars() {
        let hex_value = format!("{:04x}", ch as u32);
        hex_values.push(hex_value);
    }
    format!("\\u{}", hex_values.join("\\u"))
}

/// Decodes a hexadecimal-encoded string into its original string representation.
///
/// # Arguments
///
/// * `value` - The hexadecimal-encoded string to be decoded.
///
/// # Returns
///
/// A string representing the decoded content.
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

/// Generates a random key with a length of 64 characters from the specified character set.
///
/// # Returns
///
/// A randomly generated key.
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
