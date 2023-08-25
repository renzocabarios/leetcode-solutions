## longest-substring-without-repeating-characters

## REFERENCE

- https://leetcode.com/problems/longest-substring-without-repeating-characters/

## SOLUTION

``` javascript
function lengthOfLongestSubstring(s: string): number {
    if (!s) return 0;
    let start = 0;
    let end = 0;
    let maxLength = 0;
    const uniqueCharacters = new Set();
    while (end < s.length) {
        if (!uniqueCharacters.has(s[end])) {
            uniqueCharacters.add(s[end]);
            end++;
            maxLength = Math.max(maxLength, uniqueCharacters.size);
        } else {
            uniqueCharacters.delete(s[start]);
            start++;
        }
    }
    return maxLength;
};
```


## TIMESTAMP

- 8/23/2023 00:14:00 
- 8/24/2023 00:05:02 
- 8/25/2023 00:06:02 
