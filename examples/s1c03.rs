extern crate matasano;
extern crate rustc_serialize;

use matasano::CanScore;
use rustc_serialize::hex::FromHex;
use std::{str,f64};
use matasano::freq_scorer::FreqScorer;


fn main() {

    let f = FreqScorer::new("data/char_likelihoods").unwrap();
    let x = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
    
    let mut max_score : f64 = f64::MIN;
    let mut max_char = 0;


    let mut max_sent : String = "".to_string();

    for char in 0..255 {
       let y : Vec<u8> = x.iter().map(|x| x ^ char).collect();       

       match str::from_utf8(&y) {
            Ok(x) => { 
                      if f.score(x) > max_score {
                          max_score = f.score(x);
                          max_char = char;
                          max_sent = x.to_string();

                      } 
                     },
            _ => continue,
        }
    } 
    
    println!("{} {} {}", max_score, max_char, max_sent);
}
