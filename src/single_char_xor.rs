use super::CanScore;
use std::{str,f64};

pub fn decrypt(buf: &[u8], key: u8) -> Vec<u8> {
   return buf.iter().map(|x| x ^ key).collect::<Vec<u8>>(); 
}

pub fn brute_force<T: CanScore>(f: &T, buf: &[u8]) -> u8 {
    let mut max_score = f64::MIN;
    let mut max_char = 0;

    for char in 0..255 {
        let y = decrypt(buf, char);
        match str::from_utf8(&y) {
            Ok(x) => {
                      let score = f.score(x);
                      if score > max_score {
                          max_score = f.score(x);
                          max_char = char;
                      }
                     },
             _ => continue,
        }              
    }
    return max_char;
}
