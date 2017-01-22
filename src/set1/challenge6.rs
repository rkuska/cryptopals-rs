use std::f32;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate itertools;
use self::itertools::Itertools;

use set1::challenge2::fixed_xor;
use set1::challenge3::find_original;


fn base64_to_bytes(base64: &[u8]) -> Vec<u8> {
    let mut output:Vec<u8> = Vec::new();
    let base64_map =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    for chunk in base64.chunks(4) {
        let mut b = Vec::new();
        for _chunk in chunk.iter() {
            b.push(base64_map.find(*_chunk as char).expect("Yo dawg input some base64 chars"));
        }
        output.push((b[0] << 2 | b[1] >> 4) as u8);
        if b[2] < 64 {
            output.push((b[1] << 4 | b[2] >> 2) as u8);
            if b[3] < 64 {
                output.push((b[2] << 6 | b[3]) as u8);
            }
        }
    }
    output
}


fn hamming_distance(first: &[u8], second: &[u8]) -> i32 {
    fixed_xor(first, second).iter().map(|x| x.count_ones() as i32).sum()
}


fn compute_weight(keysize: usize, bytes: &[u8]) -> f32 {
     let bytes_pair = bytes.chunks(keysize).take(4).combinations(2);
     bytes_pair.map(|pair| hamming_distance(&pair[0], &pair[1]) as f32 / keysize as f32).sum()
}


fn find_likely_keysize(text: &[u8], max: usize) -> Option<(usize, f32)>{
    (1..max).map(|x| (x, compute_weight(x, text))).min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
}


fn transpose(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut transposed:Vec<Vec<u8>> = Vec::new();
    for _ in 0..matrix[0].len() {
        transposed.push(Vec::new());
    }
    for row in matrix {
        for (column, byte) in row.iter().enumerate() {
            transposed[column].push(*byte);
        }
    }
    transposed
}


fn load_file(path: &str) -> Vec<u8> {
    let file = BufReader::new(File::open(path).expect("Error when reading file <challenge 6>"));
    file.lines().flat_map(|line| line.unwrap().into_bytes()).collect::<Vec<_>>()
}

fn decrypt_repeated_xor(base64: &[u8]) -> String {
    let bytes = base64_to_bytes(base64);
    let keysize = find_likely_keysize(&bytes, 40).unwrap().0;
    let transposed = transpose(bytes.chunks(keysize).map(|chunk| chunk.to_vec()).collect::<Vec<_>>());
    let transposed_text:Vec<Vec<u8>> = transposed
        .iter()
        .map(|x| find_original(&x).unwrap().1.into_bytes())
        .collect::<Vec<_>>();
    String::from_utf8(transpose(transposed_text.clone()).into_iter().flat_map(|vec| vec).collect::<Vec<_>>()).unwrap()
}

fn decrypt_repeated_xor_from_file(path: &str) -> String {
    decrypt_repeated_xor(&load_file(path))
}

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()), 37)
}


#[test]
fn test_compute_weight() {
    assert_eq!(compute_weight(8, "this is a test".as_bytes()), 2.5)
}

#[test]
fn test_find_keysize() {
    assert_eq!(find_likely_keysize("dsklasleiqw456".as_bytes(), 4).unwrap(), (3, 15.333334))
}

#[test]
fn test_decrypt_repeated_xor() {
    assert!(decrypt_repeated_xor_from_file("./6.txt").starts_with("I'm back and I'm ringin"));
}

#[test]
fn test_base64_to_bytes() {
    base64_to_bytes("ASDLJFHDKKSJDFSFDSDS".as_bytes());
}

#[test]
fn test_transpose() {
    assert_eq!(transpose(vec![vec![1,2,3], vec![4,5,6]]), vec![vec![1,4],vec![2,5],vec![3,6]])
}
