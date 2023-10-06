## longest-substring-without-repeating-characters

## REFERENCE

- https://leetcode.com/problems/longest-substring-without-repeating-characters/

## SOLUTION

``` typescript
function lengthOfLongestSubstring(s: string): number {
    if(!s) return 0;
    
    let start: number = 0;
    let end: number = 0;
    let max: number = 0;
    const unique = new Set();

    while(end < s.length){
        if(!unique.has(s[end])){
            unique.add(s[end]);
            end++;
            max = Math.max(max, unique.size);
        }else{
            unique.delete(s[start]);
            start++;
        }
    }

    return max;
};
```


## TIMESTAMP

- 8/23/2023 00:14:00 
- 8/24/2023 00:05:02 
- 8/25/2023 00:06:02 
- 8/26/2023 00:06:19
- 8/27/2023 00:02:52
- 8/28/2023 00:04:30
- 9/4/2023 00:04:06
- 09/14/2023 00:03:13
- 10/02/2023 00:04:54
- 10/03/2023 00:02:39
- 10/06/2023 00:02:39








