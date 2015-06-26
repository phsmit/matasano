extern crate rustc_serialize;

pub mod freq_scorer;
pub mod single_char_xor;
pub mod s1c01;
pub mod s1c02;
pub mod s1c05;

pub trait CanScore {
    fn score(&self, &str) -> f64;
}

pub fn hamming_distance(l: &[u8], r: &[u8]) -> u64 {
    let mut dist = 0;
    for (le, re) in l.iter().zip(r.iter()) {
        let mut diff = *le ^ *re;
	while diff != 0 {
            dist += 1;
            diff &= diff - 1;
        }
    }
    return dist;
}

#[test]
fn test_hamming_distance() {
    assert_eq!(37, hamming_distance(b"this is a test", b"wokka wokka!!!"));
}
