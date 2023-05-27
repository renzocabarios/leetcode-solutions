## jewels-and-stones

## REFERENCE

- https://leetcode.com/problems/jewels-and-stones/

## SOLUTION

``` typescript
function numJewelsInStones(jewels: string, stones: string): number {
    let temp = 0;
    const tempJewel = jewels.split("");
    stones.split("").forEach((stone: string) => {
        if(tempJewel.includes(stone)){
            temp++;
        }
    })

    return temp
};
```


## TIMESTAMP

- 5/26/2023 00:17:14

