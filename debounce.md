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

