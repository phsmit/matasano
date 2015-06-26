
pub fn repeat_key_xor(key: &[u8], text: &mut [u8]) { 
    for (t, k) in text.iter_mut().zip(key.iter().cycle()) {
        *t ^= *k;
    }
}
