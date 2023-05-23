## build-array-from-permutation

## REFERENCE

- https://leetcode.com/problems/build-array-from-permutation/

## SOLUTION

``` javascript
function buildArray(nums: number[]): number[] {
    let temp = [];

    nums.forEach((e, index) => {
        temp.push(nums[nums[index]]);
    })

    return temp
};
```


## TIMESTAMP

- 5/20/2023 00:00:00 
- 5/18/2023 00:01:01 
- 5/21/2023 00:01:30 
- 5/24/2023 00:01:59
