## left-and-right-sum-differences

## REFERENCE

- https://leetcode.com/problems/left-and-right-sum-differences/

## SOLUTION

``` Typescript
function leftRightDifference(nums: number[]): number[] {
    let ans = [];

    let right = nums.reduce((acc, curr) => acc + curr, 0)
    let left = 0

    nums.forEach((e: number) => {
        right -= e;
        ans.push(Math.abs(left-right));
        left += e;

    })

    return ans;
};
```

## TIMESTAMP

- 6/06/2023 00:12:10