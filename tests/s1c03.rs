extern crate matasano;

use matasano::bit::key_xor;
use matasano::io::hex_to_bytes;
use matasano::freq_scorer::FreqScorer;
use matasano::single_char_xor::brute_force;

use std::str;

#[test]
fn test_s1c03() {
    let input = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let key = brute_force(&FreqScorer::new("data/char_likelihoods").unwrap(),
                          &input,
                          32..126);

    let result = key_xor(&input, &vec![key]);
    let result = str::from_utf8(&result).unwrap();

    assert_eq!("Cooking MC's like a pound of bacon",
               result);
}
