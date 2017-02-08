extern crate rand;

use self::rand::Rng;
use self::rand::distributions::{Range, IndependentSample};

use set2::challenge10::encrypt_aes_cbc;
use set1::challenge7::encrypt_aes_ecb;

fn encryption_oracle(input: &[u8]) -> Vec<u8> {
    let extra_bytes_range = Range::new(5, 11);
    let mut rng = rand::thread_rng();
    let key = random_key();
    let nr_extra_bytes = extra_bytes_range.ind_sample(&mut rng);
    let plaintext = [&vec![8; nr_extra_bytes], &input[..], &vec![8; nr_extra_bytes]].concat();
    if rng.gen() {
       encrypt_aes_ecb(&plaintext, &key).unwrap()
    }
    else {
       encrypt_aes_cbc(&plaintext, &key, rng.gen::<u8>())
    }
}

fn random_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key = Vec::new();
    for _ in 0..16 {
        key.push(rng.gen::<u8>());
    }
    key
}
