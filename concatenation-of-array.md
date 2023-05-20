## concatenation-of-array

## REFERENCE

- https://leetcode.com/problems/concatenation-of-array/

## SOLUTION

``` Typescript
function getConcatenation(nums: number[]): number[] {
    let ans: number[] = [...nums, ...nums];
    return ans;
};
```

## TIMESTAMP

- 5/19/2023 00:05:20
- 5/20/2023 00:01:20
