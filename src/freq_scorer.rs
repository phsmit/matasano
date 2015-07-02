use super::CanScore;

use std::{fs,io,f64};
use std::io::prelude::*;

const LOW_A: u8 = 97;
const LOW_Z: u8 = 122;
const UPP_A: u8 = 65;
const UPP_Z: u8 = 90;
const SPACE: u8 = 32;


pub struct FreqScorer {
    freq: [f64; 28],
}

impl CanScore for FreqScorer {
    fn score(&self, input: &[u8]) -> f64 {
        let mut score = 0.0;
        for &char in input {
            score += match char {
                LOW_A...LOW_Z => self.freq[(char - LOW_A) as usize],
                UPP_A...UPP_Z => self.freq[(char - UPP_A) as usize],
                SPACE => self.freq[26],
                _ => self.freq[27],
            }
        }
        return score;
    }
}

impl FreqScorer {
    pub fn new(path: &str) -> io::Result<FreqScorer> {
        let mut scorer = FreqScorer{freq: [f64::MIN; 28]};
        let file = try!(fs::File::open(path));
        for line in io::BufReader::new(file).lines() {
            let line = try!(line);
            if line.chars().next().unwrap() == '#' {
                continue;
            }
            let parts : Vec<&str> = line.splitn(2, '|').collect();
            let k = parts[0].chars().next().unwrap();
            let v: f64 = parts[1].parse().unwrap();

            match k {
                'a'...'z' => scorer.freq[k as usize - 'a' as usize] = v,
                ' ' => scorer.freq[26] = v,
                _ => scorer.freq[27] = v,
            }
        }
    	return Ok(scorer);
    }
}
