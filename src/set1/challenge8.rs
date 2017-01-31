use set1::challenge6::compute_weight;
use set1::utils::hex_to_bytes;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn find_aes_ecb_ciphered(path: &str) -> Option<(f32, String)> {
    let file = BufReader::new(File::open(path).expect("Error when reading file <challenge 4>"));
    file.lines()
        .map(|line| {
            let unwrapped = line.unwrap();
            let bytes = hex_to_bytes(&unwrapped);
            (compute_weight(16, &bytes), unwrapped)
        })
        .min_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
}


#[test]
fn test_find_aes_ecb() {
    assert_eq!(find_aes_ecb_ciphered("./files/challenge-8.txt").unwrap().1,
               "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a");
}
