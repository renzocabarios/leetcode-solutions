use crate::contains_duplicate;

#[test]
fn example1() {
    let nums: Vec<i32> = vec![1, 2, 3, 1];
    let result = contains_duplicate(nums);
    assert_eq!(result, true);
}
#[test]
fn example2() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    let result = contains_duplicate(nums);
    assert_eq!(result, false);
}
#[test]
fn example3() {
    let nums: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    let result = contains_duplicate(nums);
    assert_eq!(result, true);
}
