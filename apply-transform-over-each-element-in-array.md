## apply-transform-over-each-element-in-array

## REFERENCE

- https://leetcode.com/problems/apply-transform-over-each-element-in-array/

## SOLUTION

``` Typescript
function map(arr: number[], fn: (n: number, i: number) => number): number[] {
    let temp = []
    arr.forEach((e: number, index: number) =>  temp.push(fn(e, index)));
    return temp;
};

```

## TIMESTAMP

- 5/19/2023 00:07:27
- 5/19/2023 00:02:17
- 5/23/2023 00:01:29
- 6/01/2023 00:01:25
