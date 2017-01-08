use std::iter::repeat;

use set1::challenge2::fixed_xor;
use set1::utils::bytes_to_hex;

fn repeating_xor(text: &[u8], key: &[u8]) -> String {
    bytes_to_hex(&fixed_xor(text,
                            &(repeat(key)
                                .take(text.len() / 3 + 1)
                                .flat_map(|x| x.iter().cloned())
                                .collect::<Vec<_>>())))

}


#[test]
fn test_repeating_xor() {
    assert_eq!(repeating_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes(),
                             "ICE".as_bytes()),
               "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a31\
               24333a653e2b2027630c692b20283165286326302e27282f")
}
