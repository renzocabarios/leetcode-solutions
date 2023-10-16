## kids-with-the-greatest-number-of-candies

## REFERENCE

- https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/

## SOLUTION

``` typescript
function kidsWithCandies(candies: number[], extraCandies: number): boolean[] {
    const max = Math.max(...candies);
    return candies.map((e: number) => (e + extraCandies >= max) )
};
```


## TIMESTAMP

- 6/03/2023 00:25:00
- 6/04/2023 00:07:21
- 6/07/2023 00:02:39
- 6/15/2023 00:01:43
- 8/1/2023 00:03:51
