extern crate matasano;
extern crate rustc_serialize;

use matasano::sentence_prob;
use rustc_serialize::hex::FromHex;
use std::str;
use std::f64;
use std::io::prelude::*;
use std::io;
use std::fs::File;

fn main() {
    let file = File::open("data/s1c04").unwrap();

    let mut max_score : f64 = f64::MIN;
    let mut max_char = 0;

    let mut max_sent : String = String::from("");

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();    
        let x = line.from_hex().unwrap();
    

        for char in 0..255 {
            let y : Vec<u8> = x.iter().map(|x| x ^ char).collect();       

           match str::from_utf8(&y) {
                Ok(x) => { 
                          if sentence_prob(x) > max_score {
                              max_score = sentence_prob(x);
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
