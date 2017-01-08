use set1::challenge2::fixed_xor;
use set1::utils::hex_to_bytes;

pub fn chi_squared(text: &str) -> f32 {
    let mut chars_count = vec![0.0; 26];
    let mut ignored = 0;
    for byte in text.bytes() {
        if (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122) {
            chars_count[if byte >= 97 {
                (byte - 97) as usize
            } else {
                (byte - 65) as usize
            }] += 1.0;
        }
        // space
        else if byte != 32 {
            ignored += 1;
        }
    }
    // ascii only hence len usage
    let length = (text.len() - ignored) as f32;
    vec![0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
         0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
         0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074]
        .iter()
        .zip(chars_count.iter())
        .map(|(x, y)| (y - x * (text.len() as f32 + ignored as f32)).powf(2.0) / (x * length))
        .sum()
}

pub fn find_original(text: &str) -> Option<(f32, String)>{
    (0..255).filter_map(|x| {
            let cipher = vec![x; text.len()];
            let xor = fixed_xor(&hex_to_bytes(text), &cipher);
            String::from_utf8(xor).ok()
        })
        .map(|x| (chi_squared(&x), x))
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
}

#[test]
fn test_chi_squared_upper_a() {
    assert_eq!(chi_squared("A"), 11.244388)
}

#[test]
fn test_chi_squared_a() {
    assert_eq!(chi_squared("a"), 11.244388)
}

#[test]
fn test_find_original() {
    assert_eq!(find_original("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap().1,
               String::from("Cooking MC\'s like a pound of bacon"))
}
