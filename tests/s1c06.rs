extern crate matasano;

use matasano::io;
use matasano::{guess_block_size,freq_scorer,bit};
use matasano::single_char_xor::brute_force;


use std::{fs,str};
use std::io::prelude::*;


fn get_data() -> Vec<u8> {
    let mut file = fs::File::open("data/s1c06").unwrap();
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf).unwrap();
    io::base64_to_bytes(&buf)
}

#[test]
fn test_s1c06_bs() {
    assert_eq!(29, guess_block_size(&get_data(), 2..40) );
}

#[test]
fn test_s1c06_result() {
    let f = freq_scorer::FreqScorer::new("data/char_likelihoods").unwrap();
    let input = get_data();
    let bs = guess_block_size(&input, 2..40);
    let key : Vec<u8> = bit::transpose(&input, bs).iter().map(|x| brute_force(&f, x, 0..255)).collect();

    assert_eq!("Terminator X: Bring the noise", str::from_utf8(&key).unwrap());
}
