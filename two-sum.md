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
- 6/01/2023 00:07:41
- 6/02/2023 00:05:41
- 6/03/2023 00:02:53
- 6/06/2023 00:02:02
- 6/13/2023 00:01:35
- 7/26/2023 00:03:38
- 7/28/2023 00:02:02
- 8/22/2023 00:02:37
