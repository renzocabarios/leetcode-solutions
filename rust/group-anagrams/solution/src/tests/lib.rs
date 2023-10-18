use crate::group_anagrams;

#[test]
fn example1() {
    // let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
    //     .iter()
    //     .map(|&s| s.to_string())
    //     .collect();
    // let result = group_anagrams(strs);
    // assert_eq!(
    //     result,
    //     vec![vec!["tan", "nat"], vec!["bat"], vec!["eat", "tea", "ate"]]
    // );
}

#[test]
fn example2() {
    let strs: Vec<String> = vec![""].iter().map(|&s| s.to_string()).collect();
    let result = group_anagrams(strs);
    assert_eq!(result, vec![vec![""]]);
}

#[test]
fn example3() {
    let strs: Vec<String> = vec!["a"].iter().map(|&s| s.to_string()).collect();
    let result = group_anagrams(strs);
    assert_eq!(result, vec![vec!["a"]]);
}
