
extern crate matasano;
use matasano::io::{bytes_to_base64,hex_to_bytes};

#[test]
fn test_s1c01() {
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
               bytes_to_base64(&hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));
}
