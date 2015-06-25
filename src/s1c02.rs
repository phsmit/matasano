
use rustc_serialize::hex::{FromHex,ToHex};

pub fn fixed_xor(l: &str, r: &str) -> String {
    let mut l = l.from_hex().unwrap();
    let r = r.from_hex().unwrap();
    for i in 0..l.len() {
        l[i] ^= r[i];
    }
    return l.to_hex();
}
