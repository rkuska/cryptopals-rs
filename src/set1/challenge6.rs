use set1::challenge2::fixed_xor;

fn hamming_distance(first: &str, second: &str) -> i32 {
    fixed_xor(first.as_bytes(), second.as_bytes()).iter().map(|x| x.count_ones() as i32).sum()
}


#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37)
}
