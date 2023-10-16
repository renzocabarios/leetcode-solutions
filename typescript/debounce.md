## debounce

## REFERENCE

- https://leetcode.com/problems/debounce/

## SOLUTION

``` javascript
type F = (...p: any[]) => any

function debounce(fn: F, t: number): F {
    let id;
    return function(...args) {
        clearTimeout(id);
        id = setTimeout(() => fn(...args), t)
    }
};

/**
 * const log = debounce(console.log, 100);
 * log('Hello'); // cancelled
 * log('Hello'); // cancelled
 * log('Hello'); // Logged at t=100ms
 */
```


## TIMESTAMP

- 5/22/2023 00:07:36
- 5/23/2023 00:05:29
- 5/24/2023 00:01:39
- 5/25/2023 00:00:55
- 5/28/2023 00:00:44
- 6/02/2023 00:00:58
- 6/15/2023 00:01:08
- 8/22/2023 00:02:08
