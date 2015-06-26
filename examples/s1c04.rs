extern crate matasano;
extern crate rustc_serialize;

use matasano::CanScore;
use matasano::freq_scorer::FreqScorer;
use rustc_serialize::hex::FromHex;

use std::{f64,fs,io,str};
use std::io::prelude::*;

fn main() {
    let f = FreqScorer::new("data/char_likelihoods").unwrap();
    let file = fs::File::open("data/s1c04").unwrap();

    let mut max_score : f64 = f64::MIN;
    let mut max_char = 0;

    let mut max_sent : String = "".to_string(); 

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();    
        let x = line.from_hex().unwrap();
    

        for char in 0..255 {
            let y : Vec<u8> = x.iter().map(|x| x ^ char).collect();       

           match str::from_utf8(&y) {
                Ok(x) => { 
                          if f.score(x) > max_score {
                              max_score = f.score(x);
                              max_char = char;
                              max_sent = String::from(x);
                          } 
                         },
                _ => continue,
            }
        }
    } 
    

    
    println!("{} {} {}", max_score, max_char, max_sent);

}
