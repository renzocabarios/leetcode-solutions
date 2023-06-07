## jewels-and-stones

## REFERENCE

- https://leetcode.com/problems/jewels-and-stones/

## SOLUTION

``` typescript
function numJewelsInStones(jewels: string, stones: string): number {
    return stones.split("").reduce((acc, curr) => jewels.includes(curr) ? acc + 1: acc, 0);
};
```


## TIMESTAMP

- 5/26/2023 00:17:14
- 5/28/2023 00:02:58
- 6/01/2023 00:02:52
- 6/08/2023 00:03:36
