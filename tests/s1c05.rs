extern crate matasano;
extern crate rustc_serialize;

use rustc_serialize::hex::ToHex;
use matasano::s1c05::repeat_key_xor;

#[test]
fn test_s1c05() {
    let text: &[u8] = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let mut result: Vec<u8> = text.iter().map(|x| *x).collect();
    
    repeat_key_xor("ICE".as_bytes(), &mut result);
    
    assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
               result.to_hex()); 

}

