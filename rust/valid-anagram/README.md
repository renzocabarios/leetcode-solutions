## valid-anagram

## REFERENCE

- https://leetcode.com/problems/valid-anagram/

## SOLUTION

``` Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if (t.len() != s.len()){
            return false;
        }
        
        let mut map: HashMap<char, i64> = HashMap::new();
        
        for (a, b) in s.chars().zip(t.chars()){
            *map.entry(a).or_default() += 1;
            *map.entry(b).or_default() -= 1;
        }
        
        map.into_values().all(|cnt| cnt == 0)
    }
}
```

## TIMESTAMP

- 10/16/2023 00:00:00
