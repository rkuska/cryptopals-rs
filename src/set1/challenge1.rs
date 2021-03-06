use set1::utils::hex_to_bytes;


fn hex_to_base64(input: &str) -> String {
    bytes_to_base64(&hex_to_bytes(input))
}


fn bytes_to_base64(u8_input: &[u8]) -> String {
    let mut output = String::new();
    let base64_map: Vec<_> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();


    for chunk in u8_input.chunks(3) {
        let c0 = chunk[0];
        let b = (c0 & 0xFC) >> 2;
        output.push(base64_map[b as usize]);
        let mut b = (c0 & 0x03) << 4;

        if let Some(c1) = chunk.get(1) {
            b |= (c1 & 0xF0) >> 4;
            output.push(base64_map[b as usize]);
            let mut b = (c1 & 0x0F) << 2;
            if let Some(c2) = chunk.get(2) {
                b |= (c2 & 0xC0) >> 6;
                output.push(base64_map[b as usize]);
                let b = c2 & 0x3F;
                output.push(base64_map[b as usize]);
            } else {
                output.push(base64_map[b as usize]);
                output.push('=');
            }
        } else {
            output.push(base64_map[b as usize]);
            output.push_str("==");
        }

    }
    output
}

#[test]
fn test_hex_to_base64() {
    let first = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!(hex_to_base64(first),
               "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
