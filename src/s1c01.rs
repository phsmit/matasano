
use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::{ToBase64, STANDARD};

pub fn hex_to_base64(h: &str) -> String {
   let x = h.from_hex().unwrap();
   return x.to_base64(STANDARD); 
}
