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
- 5/25/2023 00:02:08
- 5/28/2023 00:01:44
- 6/02/2023 00:03:34
- 6/13/2023 00:04:04
- 8/22/2023 00:02:07
