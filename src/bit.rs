

pub fn key_xor(data: &[u8], key:&[u8]) -> Vec<u8> {
    data.iter().zip(key.iter().cycle()).map(|(d,k)| *d ^ *k).collect()
}

pub fn transpose(data: &[u8], stride: usize) -> Vec<Vec<u8>> {
    let mut result : Vec<Vec<u8>> = vec![vec![];stride];

    for (i, d) in data.iter().enumerate() {
        result[i%stride].push(*d);
    }

    return result;
}

#[test]
fn test_transpose() {
    assert_eq!(vec![vec![1,4,7], vec![2,5,8], vec![3,6]],
               transpose(&vec![1,2,3,4,5,6,7,8], 3));
}
