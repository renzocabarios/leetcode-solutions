## TWO SUM

## REFERENCE

- https://leetcode.com/problems/two-sum/

## SOLUTION

``` javascript
function twoSum(nums: number[], target: number): number[] {
    const length = nums.length;
    const sums = [];
    for(var i = 0; i < length-1; i++){
        for(var j = i + 1; j < length; j++){
            if(nums[i] + nums[j] == target){
                return [i, j]
            }
        }
    }
    return [];
};
```


## TIMESTAMP

- 5/17/2023 STARTED
- 5/18/2023 00:06:19
- 5/21/2023 00:06:27
