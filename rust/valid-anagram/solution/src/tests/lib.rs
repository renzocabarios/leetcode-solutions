use crate::is_anagram;

#[test]
fn example1() {
    let s: String = "anagram".to_string();
    let t: String = "nagaram".to_string();
    let result = is_anagram(s, t);
    assert_eq!(result, true);
}

#[test]
fn example2() {
    let s: String = "rat".to_string();
    let t: String = "car".to_string();
    let result = is_anagram(s, t);
    assert_eq!(result, false);
}
