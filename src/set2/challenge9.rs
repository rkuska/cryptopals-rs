fn pkcs7padding(plaintext: &[u8], blocksize: usize) -> Vec<u8> {
    let padding_length = blocksize - (plaintext.len() % blocksize);
    let padding = vec![padding_length as u8; padding_length];
    [&plaintext[..], &padding[..]].concat()
}


#[test]
fn test_pkcs7padding() {
    assert_eq!(String::from_utf8(pkcs7padding("YELLOW SUBMARINE".as_bytes(), 20)).unwrap(), "YELLOW SUBMARINE\x04\x04\x04\x04")
}
