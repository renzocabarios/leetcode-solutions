## how-many-numbers-are-smaller-than-the-current-number

## REFERENCE

- https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

## SOLUTION

``` Typescript
function smallerNumbersThanCurrent(nums: number[]): number[] {
    const answer: number[] = []

    for(let i: number = 0; nums.length > i; i++){
        let count: number = 0;
        for(let j: number = 0; nums.length > j; j++){
            if(nums[i] > nums[j]){
                count++;
            }
        }
        answer.push(count)
    }
    return answer;
};
```

## TIMESTAMP

- 09/13/2023 00:03:29
- 09/14/2023 00:03:01
- 09/22/2023 00:02:28




















