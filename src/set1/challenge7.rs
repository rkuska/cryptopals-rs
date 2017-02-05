extern crate crypto;

use self::crypto::aes::{KeySize, ecb_decryptor, ecb_encryptor};
use self::crypto::blockmodes;
use self::crypto::buffer::{RefReadBuffer, ReadBuffer, RefWriteBuffer, WriteBuffer, BufferResult};
use self::crypto::symmetriccipher;

use set1::challenge6::{base64_to_bytes, load_file};


pub fn decrypt_aes_ecb(encrypted_data: &[u8],
                   key: &[u8])
                   -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = ecb_decryptor(KeySize::KeySize128, key, blockmodes::NoPadding);
    let mut read_buffer = RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut output = Vec::<u8>::new();
    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        output.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    Ok(output)
}


pub fn encrypt_aes_ecb(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = ecb_encryptor(KeySize::KeySize128, key, blockmodes::NoPadding);
    let mut read_buffer = RefReadBuffer::new(plaintext);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut output = Vec::<u8>::new();
    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        output.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    Ok(output)
}


#[test]
fn test_decrypt_aes_ecb() {
    assert!(String::from_utf8(decrypt_aes_ecb(&base64_to_bytes(&load_file("./files/challenge-7.\
                                                                           txt")),
                                              &"YELLOW SUBMARINE".as_bytes())
            .unwrap())
        .unwrap()
        .starts_with("I\'m back and I\'m ringin"));

}

#[test]
fn test_encrypt_aes_ecb() {
    assert_eq!(encrypt_aes_ecb(&decrypt_aes_ecb(&base64_to_bytes(&load_file("./files/challenge-7.\
                                                                           txt")),
                                              &"YELLOW SUBMARINE".as_bytes())
            .unwrap(), &"YELLOW SUBMARINE".as_bytes()).unwrap(),
        base64_to_bytes(&load_file("./files/challenge-7.txt")));
}
