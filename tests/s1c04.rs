extern crate matasano;
extern crate rustc_serialize;

use matasano::CanScore;
use matasano::freq_scorer::FreqScorer;
use matasano::io::hex_to_bytes;

use std::{f64,fs,io,str};
use std::io::prelude::*;

#[test]
fn test_s1c04() {
    let f = FreqScorer::new("data/char_likelihoods").unwrap();
    let file = fs::File::open("data/s1c04").unwrap();

    let mut max_score : f64 = f64::MIN;

    let mut max_sent : Vec<u8> = vec![];

    for line in io::BufReader::new(file).lines() {
        let x = hex_to_bytes(&line.unwrap());

        for char in 0..255 {
            let y : Vec<u8> = x.iter().map(|x| x ^ char).collect();

            let score = f.score(&y);
            if score > max_score {
              max_score = score;
              max_sent = y.clone();
            }
        }
    }

    let max_sent = str::from_utf8(&max_sent).unwrap();

    assert_eq!("Now that the party is jumping\n",
               max_sent);
}
