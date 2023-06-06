## number-of-good-pairs

## REFERENCE

- https://leetcode.com/problems/number-of-good-pairs

## SOLUTION

``` Typescript
function numIdenticalPairs(nums: number[]): number {
    let temp =0;
    for(let i = 0; i < nums.length-1; i++){
        for(let j = i+1; j < nums.length; j++){
            if(nums[i] === nums[j] && i < j){
                temp++;
            }
        }
    }
    return temp
};
```

## TIMESTAMP

- 6/03/2023 00:03:26
- 6/04/2023 00:03:10
- 6/07/2023 00:05:07
