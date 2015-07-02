use super::CanScore;
use std::f64;

pub fn decrypt(buf: &[u8], key: u8) -> Vec<u8> {
   return buf.iter().map(|x| x ^ key).collect::<Vec<u8>>();
}

pub fn brute_force<T,I>(f: &T, buf: &[u8], candidates: I) -> u8 where T: CanScore, I: Iterator<Item=u8> {
    let mut max_score = f64::MIN;
    let mut max_char = 0;

    for c in candidates {
        let score = f.score(&decrypt(buf, c));
        if score > max_score {
            max_score = score;
            max_char = c;
        }
    }

    return max_char;
}
