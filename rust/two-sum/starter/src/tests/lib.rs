use crate::two_sum;

#[test]
fn example1() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result = two_sum(nums, target);

    assert_eq!(result, vec![1, 0]);
}

#[test]
fn example2() {
    let nums: Vec<i32> = vec![3, 2, 4];
    let target: i32 = 6;
    let result = two_sum(nums, target);
    assert_eq!(result, vec![2, 1]);
}

#[test]
fn example3() {
    let nums: Vec<i32> = vec![3, 3];
    let target: i32 = 6;
    let result = two_sum(nums, target);
    assert_eq!(result, vec![1, 0]);
}
