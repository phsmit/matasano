extern crate matasano;
extern crate rustc_serialize;

use std::{f64,fs,io,str};
use std::io::prelude::*;
use matasano::{CanScore,single_char_xor,hamming_distance,freq_scorer,s1c05};

use rustc_serialize::base64::FromBase64;

const TEST_BLOCKS:usize = 3;

fn distance(buf: &[u8], key_size: usize) -> f64 {
    let mut dist = 0.0; 
    for i in 0..TEST_BLOCKS {
        dist += hamming_distance(&buf[i*key_size..(i+1)*key_size], &buf[(i+1)*key_size..(i+2)*key_size]) as f64
    }
    dist = dist / (TEST_BLOCKS * key_size) as f64;
    return dist;
}

fn main() {
    let f = freq_scorer::FreqScorer::new("data/char_likelihoods").unwrap();
    let mut file = fs::File::open("data/s1c06").unwrap();
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf).unwrap();
    let mut input = buf.from_base64().unwrap();
 
    let mut min_dist = f64::MAX;
    let mut min_key = 0;

    for key_size in 2..50 {
        let dist = distance(&input, key_size);
        if dist < min_dist {
            min_dist = dist;
            min_key = key_size;
        }
    }   
    
    let mut key = Vec::<u8>::new();
    for i in 0..min_key {
        let buf : Vec<u8> = input.iter().enumerate().filter_map(|(j, v)| if j % min_key == i {Some(*v)} else {None}).collect();
        key.push(single_char_xor::brute_force(&f, &buf));
    }
    
    s1c05::repeat_key_xor(&key, &mut input);
    let result = str::from_utf8(&input).unwrap();
    let key = str::from_utf8(&key).unwrap();
    println!("{}\n{}", key, result);
println!("{}",input.len());
}
