## kids-with-the-greatest-number-of-candies

## REFERENCE

- https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/

## SOLUTION

``` typescript
function kidsWithCandies(candies: number[], extraCandies: number): boolean[] {
    let result = [];

    const max: number = candies.reduce((acc, curr) => {
        return curr > acc ? curr : acc;
    }, 0);
    

    for(let i = 0; i < candies.length; i++) {
        result.push(candies[i] + extraCandies >= max) 
    }
    
    return result
};
```


## TIMESTAMP

- 6/03/2023 00:25:00
