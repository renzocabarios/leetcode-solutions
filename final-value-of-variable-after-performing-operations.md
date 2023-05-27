## final-value-of-variable-after-performing-operations

## REFERENCE

- https://leetcode.com/problems/final-value-of-variable-after-performing-operations/

## SOLUTION

``` typescript
function finalValueAfterOperations(operations: string[]): number {
    let init = 0;
    operations.forEach((e: string) => {
        if(e.includes("-")) init--;
        if(e.includes("+")) init++;
    })
    return init;
};
```


## TIMESTAMP

- 5/26/2023 00:02:52
- 5/27/2023 00:02:10
