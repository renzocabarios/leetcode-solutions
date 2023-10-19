use crate::top_k_frequent;

#[test]
fn example1() {
    let nums: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
    let k: i32 = 2;
    let result = top_k_frequent(nums, k);
    assert_eq!(result, vec![2, 1]);
}

#[test]
fn example2() {
    let nums: Vec<i32> = vec![1];
    let k: i32 = 1;
    let result = top_k_frequent(nums, k);
    assert_eq!(result, vec![1]);
}
