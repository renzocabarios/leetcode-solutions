## final-value-of-variable-after-performing-operations

## REFERENCE

- https://leetcode.com/problems/final-value-of-variable-after-performing-operations/

## SOLUTION

``` typescript
function finalValueAfterOperations(operations: string[]): number {
    return operations.reduce((acc, curr) => {
        if(curr.includes("--")) return acc - 1;
        if(curr.includes("++")) return acc + 1;
        return acc;
    }, 0)
};
```


## TIMESTAMP

- 5/26/2023 00:02:52
- 5/27/2023 00:02:10
- 6/01/2023 00:01:54
- 6/13/2023 00:03:30
- 8/22/2023 00:03:14
