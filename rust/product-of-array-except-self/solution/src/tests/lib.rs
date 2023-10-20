use crate::product_except_self;

#[test]
fn example1() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    let result = product_except_self(nums);
    assert_eq!(result, vec![24, 12, 8, 6]);
}

#[test]
fn example2() {
    let nums: Vec<i32> = vec![-1, 1, 0, -3, 3];
    let result = product_except_self(nums);
    assert_eq!(result, vec![0, 0, 9, 0, 0]);
}
