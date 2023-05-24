## array-reduce-transformation

## REFERENCE

- https://leetcode.com/problems/array-reduce-transformation/

## SOLUTION

``` typescript
type Fn = (accum: number, curr: number) => number

function reduce(nums: number[], fn: Fn, init: number): number {
    let start = init;
    nums.forEach((e => {
        start = fn(start, e);
    }))
    return start;
};
```


## TIMESTAMP

- 5/24/2023 00:08:30
- 5/25/2023 00:01:27
