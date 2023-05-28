## jewels-and-stones

## REFERENCE

- https://leetcode.com/problems/jewels-and-stones/

## SOLUTION

``` typescript
function numJewelsInStones(jewels: string, stones: string): number {
    let temp = 0;
    stones.split("").forEach((e: string) => {
        if(jewels.split("").includes(e)) temp++
    })
    return temp;
};
```


## TIMESTAMP

- 5/26/2023 00:17:14
- 5/28/2023 00:02:58
