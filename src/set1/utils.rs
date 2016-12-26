pub fn hex_to_bytes(hex_input: &str) -> Vec<u8> {

    let hex_vec: Vec<_> = hex_input.chars().rev().collect();

    hex_vec.chunks(2).map(|chunk| {
        let second_byte = chunk[0].to_digit(16).unwrap();
        let first_byte = chunk.get(1).unwrap_or(&'0').to_digit(16).unwrap();
        (first_byte << 4 | second_byte) as u8
    }).rev().collect()

}


#[test]
fn test_hex_to_bytes() {
    assert_eq!(hex_to_bytes("ff"), [255]);
}

#[test]
fn test_hex_to_bytes_f() {
    assert_eq!(hex_to_bytes("f"), [15]);
}
