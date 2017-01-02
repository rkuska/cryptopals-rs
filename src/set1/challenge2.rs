use set1::utils::hex_to_bytes;
use set1::utils::bytes_to_hex;

pub fn fixed_xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first.iter().zip(second.iter()).map(|(x, y)| x ^ y).collect()
}

fn string_fixed_xor(first: &str, second: &str) -> String {
    bytes_to_hex(&fixed_xor(&hex_to_bytes(first), &hex_to_bytes(second)))
}

#[test]
fn test_string_fixed_xor() {
    assert_eq!(string_fixed_xor(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965"
            ), "746865206b696420646f6e277420706c6179")
}
