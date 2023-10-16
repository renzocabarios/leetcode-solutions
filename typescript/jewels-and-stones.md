## jewels-and-stones

## REFERENCE

- https://leetcode.com/problems/jewels-and-stones/

## SOLUTION

``` typescript
function numJewelsInStones(jewels: string, stones: string): number {
    return [...stones].filter((e: string) => [...jewels].includes(e)).length;
};
```


## TIMESTAMP

- 5/26/2023 00:17:14
- 5/28/2023 00:02:58
- 6/01/2023 00:02:52
- 6/08/2023 00:03:36
- 7/25/2023 00:03:32
