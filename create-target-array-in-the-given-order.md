## create-target-array-in-the-given-order

## REFERENCE

- https://leetcode.com/problems/create-target-array-in-the-given-order/

## SOLUTION

``` Typescript
function createTargetArray(nums: number[], index: number[]): number[] {
    let ans = [];
    for (let i = 0; i < nums.length; i++) {
        ans.splice(index[i], 0, nums[i]);
    }
    return ans;
};
```

## TIMESTAMP

- 09/22/2023 00:07:42
- 09/23/2023 00:14:42
- 09/24/2023 00:06:25
- 09/28/2023 00:03:48
- 10/05/2023 00:01:43
