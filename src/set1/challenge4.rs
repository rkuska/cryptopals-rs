use set1::utils::hex_to_bytes;
use set1::challenge3::find_original;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn find_line(path: &str) -> String {
    let file = BufReader::new(File::open(path).expect("Error when reading file <challenge 4>"));
    file.lines().filter_map(|x| find_original(&hex_to_bytes(&x.unwrap()))).min_by(|a, b| (a.0).partial_cmp(&b.0).unwrap()).unwrap().1
}


#[test]
fn test_find_line(){
    assert_eq!(find_line("./files/challenge-4.txt"), "Now that the party is jumping\n")
}
