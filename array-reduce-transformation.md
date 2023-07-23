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
- 5/28/2023 00:01:32
- 6/04/2023 00:02:14
- 7/23/2023 00:05:00
