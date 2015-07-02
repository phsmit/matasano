extern crate rustc_serialize;

use rustc_serialize::hex::{FromHex,ToHex};
use rustc_serialize::base64::{FromBase64,ToBase64, STANDARD};

pub fn base64_to_bytes(b: &[u8]) -> Vec<u8> {
    b.from_base64().unwrap()
}

pub fn hex_to_bytes(h: &str) -> Vec<u8> {
    h.from_hex().unwrap()
}

pub fn bytes_to_base64(b: &[u8]) -> String {
    b.to_base64(STANDARD)
}

pub fn bytes_to_hex(b: &[u8]) -> String {
    b.to_hex()
}
