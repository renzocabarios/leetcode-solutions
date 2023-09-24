## count-pairs-whose-sum-is-less-than-target

## REFERENCE

- https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/

## SOLUTION

``` Typescript
function countPairs(nums: number[], target: number): number {
    let pairs = 0;
    for(let i: number = 0; i < nums.length-0; i++){
        for(let j: number = i+1; j < nums.length; j++){
            if(nums[i] + nums[j] < target){
                pairs++;
            }
        }
    }
    return pairs
};
```

## TIMESTAMP

- 9/4/2023 00:3:10
- 9/5/2023 00:1:38
- 09/12/2023 00:04:00
- 09/24/2023 00:02:14
