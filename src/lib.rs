extern crate rustc_serialize;

pub mod s1c01;
pub mod s1c02;

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io;

fn read_char_probs(path: &str) -> Result<HashMap<char, f64>, io::Error> {
    let mut hm = HashMap::new();
    let file = try!(File::open(path));
    for line in io::BufReader::new(file).lines() {
	let line = try!(line);
        if line.chars().next().unwrap() == '#' {
            continue;
        }
	let parts : Vec<&str> = line.splitn(2, '|').collect();
        let k = parts[0].chars().next().unwrap();
        let v: f64 = parts[1].parse().unwrap();
        hm.insert(k,v);
    }
    return Ok(hm);
}

pub fn sentence_prob(sentence: &str) -> f64 {
    let hm = read_char_probs("data/char_likelihoods").unwrap();
    let default = hm.get(&'_').unwrap();
    let mut score : f64 = 0.0;
    for char in sentence.chars() {
        score += *match hm.get(&char.to_ascii_lowercase()) {
            Some(f) => f,
	    None => default, 
        }
    }
    return score;
}

#[test]
fn test_read_char_probs() {
    let hm = read_char_probs("data/char_likelihoods").unwrap();
    assert!((-2.318 - hm.get(&'e').unwrap()).abs() < 0.0001);
}

#[test]
fn test_sentence_prob() {
    assert!((-22.883 - sentence_prob("Abc e!")).abs() < 0.0001);
}
