## shuffle-the-array

## REFERENCE

- https://leetcode.com/problems/shuffle-the-array/

## SOLUTION

``` typescript
function shuffle(nums: number[], n: number): number[] {
    let temp = [];
    for(let i = 0; i < n; i++){
        temp.push(nums[i]);
        temp.push(nums[i+n]);
    }
    return temp;
};
```


## TIMESTAMP

- 5/24/2023 00:17:06
