## kids-with-the-greatest-number-of-candies

## REFERENCE

- https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/

## SOLUTION

``` typescript
function kidsWithCandies(candies: number[], extraCandies: number): boolean[] {
    const max: number = candies.reduce((acc, curr) => curr > acc ? curr : acc  , 0)
    return candies.map((e: number) => {
        return (e + extraCandies) >= max
    })
};
```


## TIMESTAMP

- 6/03/2023 00:25:00
- 6/04/2023 00:07:21
