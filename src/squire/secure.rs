extern crate base64;
extern crate sha2;

use sha2::{Digest, Sha512};

pub fn base64_decode(encoded_value: String) -> String {
    // Decode base64 and then decode UTF-8
    if let Ok(decoded_bytes) = base64::decode(&encoded_value) {  // fixme: deprecated
        if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
            decoded_str
        } else {
            panic!("Error decoding UTF-8");
        }
    } else {
        panic!("Error decoding base64");
    }
}

pub fn calculate_hash(signature: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(signature);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn hex_encode(input_str: &str) -> String {
    let mut hex_values: Vec<String> = Vec::new();
    for ch in input_str.chars() {
        let hex_value = format!("{:04x}", ch as u32);
        hex_values.push(hex_value);
    }
    format!("\\u{}", hex_values.join("\\u"))
}

pub fn hex_decode(input: &str) -> String {
    let mut result = String::new();
    let hex_values: Vec<&str> = input.split("\\u").skip(1).collect();
    for hex_value in hex_values {
        if let Ok(code_point) = u32::from_str_radix(hex_value, 16) {
            if let Some(ch) = char::from_u32(code_point) {
                result.push(ch);
            }
        }
    }
    result
}
