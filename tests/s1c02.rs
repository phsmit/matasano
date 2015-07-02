
extern crate matasano;
use matasano::io::hex_to_bytes;
use matasano::bit::key_xor;

#[test]
fn test_s1c02() {
    assert_eq!(hex_to_bytes("746865206b696420646f6e277420706c6179"),
               key_xor(&hex_to_bytes("1c0111001f010100061a024b53535009181c"),
                       &hex_to_bytes("686974207468652062756c6c277320657965")));
}
