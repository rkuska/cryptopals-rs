use set1::challenge2::fixed_xor;
use set1::challenge6::{base64_to_bytes, load_file};
use set1::challenge7::{encrypt_aes_ecb, decrypt_aes_ecb};

pub fn encrypt_aes_cbc(plaintext: &[u8], key: &[u8], iv_char: u8) -> Vec<u8> {
    let mut iv = vec![iv_char; 16];
    let encrypted = plaintext.chunks(16)
        .flat_map(|text_block| {
            let drained = iv.drain(0..).collect::<Vec<_>>();
            let xored = fixed_xor(&drained, &text_block);
            let cbc_encrypted = encrypt_aes_ecb(&xored, &key).unwrap();
            let mut cloned_cbc = cbc_encrypted.clone();
            iv.append(&mut cloned_cbc);
            cbc_encrypted
        })
        .collect::<Vec<_>>();

    encrypted
}

fn decrypt_aes_cbc(encrypted: &[u8], key: &[u8], iv_char: u8) -> Vec<u8> {
    let mut iv = vec![iv_char; 16];
    encrypted.chunks(16)
        .flat_map(|encrypted_block| {
            let ecb_decrypted = decrypt_aes_ecb(&encrypted_block, &key).unwrap();
            let drained = iv.drain(0..).collect::<Vec<_>>();
            let plaintext = fixed_xor(&drained, &ecb_decrypted);
            iv.append(&mut Vec::from(encrypted_block));
            plaintext
        })
        .collect::<Vec<_>>()

}


#[test]
fn test_decrypt_aes_cbc() {
    assert!(String::from_utf8(decrypt_aes_cbc(&base64_to_bytes(&load_file("./files/challenge-10.\
                                                                           txt")),
                                              &"YELLOW SUBMARINE".as_bytes(),
                                              0))
        .unwrap()
        .starts_with("I\'m back and I\'m ringin"));
}

#[test]
fn test_decrypt_encrypt_aes_cbc() {
    assert_eq!(encrypt_aes_cbc(&decrypt_aes_cbc(&load_file("./files/challenge-10.txt"),
                                                &"YELLOW SUBMARINE".as_bytes(),
                                                0),
                               &"YELLOW SUBMARINE".as_bytes(),
                               0),
               load_file("./files/challenge-10.txt"))
}
