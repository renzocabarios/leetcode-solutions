## partitioning-into-minimum-number-of-deci-binary-numbers

## REFERENCE

- https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/

## SOLUTION

``` typescript
function minPartitions(n: string): number {
    return Math.max(...n.split('').map((e: string) => parseInt(e)))
};
```


## TIMESTAMP

- 5/24/2023 00:13:46
- 5/25/2023 00:02:24
- 5/28/2023 00:02:27
