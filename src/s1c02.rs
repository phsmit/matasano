
use rustc_serialize::hex::{FromHex,ToHex};

pub fn fixed_xor(l: &str, r: &str) -> String {
    let mut l = l.from_hex().unwrap();
    let r = r.from_hex().unwrap();
    for (le, re) in l.iter_mut().zip(r.iter()) {
        *le ^= *re;
    }
    return l.to_hex();
}
