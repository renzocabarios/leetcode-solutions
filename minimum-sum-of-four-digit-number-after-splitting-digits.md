## minimum-sum-of-four-digit-number-after-splitting-digits

## REFERENCE

- https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits

## SOLUTION

``` Typescript
function minimumSum(num: number): number {
    const nums = new Array(4).fill(0);
    for (let i = 0; i < 4; i++) {
        nums[i] = num % 10;
        num = Math.floor(num / 10);
    }
    nums.sort((a, b) => a - b);
    return 10 * (nums[0] + nums[1]) + nums[2] + nums[3];
}
```

## TIMESTAMP

- 09/15/2023 00:07:35
- 09/21/2023 00:07:23















