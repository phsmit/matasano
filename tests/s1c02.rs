
extern crate matasano;
use matasano::s1c02::fixed_xor;

#[test]
fn test_s1c02() {
    assert_eq!("746865206b696420646f6e277420706c6179",
               fixed_xor("1c0111001f010100061a024b53535009181c",
                         "686974207468652062756c6c277320657965"));
}
