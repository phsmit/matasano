extern crate rustc_serialize;

pub mod bit;
pub mod freq_scorer;
pub mod io;
pub mod single_char_xor;

use std::usize;

pub trait CanScore {
    fn score(&self, &[u8]) -> f64;
}

pub fn hamming_distance(l: &[u8], r: &[u8]) -> usize {
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


pub fn guess_block_size<I>(b: &[u8], candidates: I) -> usize where I: Iterator<Item=usize> {
    const NUM_BLOCKS : usize = 5;

    let mut min_dist = usize::MAX;
    let mut min_key = 0;

    for key_size in candidates {
        let mut dist = 0;
        let mut blocks : Vec<&[u8]> = vec![];
        for i in 0..NUM_BLOCKS {
            blocks.push(&b[i*key_size..(i+1)*key_size]);
        }

        for i in 0..NUM_BLOCKS-1 {
            for j in i+1..NUM_BLOCKS {
                dist += hamming_distance(blocks[i], blocks[j]);
            }
        }

        dist /= key_size;

        if dist < min_dist {
            min_key = key_size;
            min_dist = dist;
        }
    }

    return min_key;
}
