## counter-ii

## REFERENCE

- https://leetcode.com/problems/counter-ii/

## SOLUTION

``` typescript
type ReturnObj = {
    increment: () => number,
    decrement: () => number,
    reset: () => number,
}

function createCounter(init: number): ReturnObj {
    let temp = init;
    return {
        increment: () => temp += 1,
        decrement: () => temp -= 1,
        reset: () => temp = init,
    }
};

/**
 * const counter = createCounter(5)
 * counter.increment(); // 6
 * counter.reset(); // 5
 * counter.decrement(); // 4
 */
```


## TIMESTAMP

- 5/25/2023 00:04:24
- 5/26/2023 00:05:26
- 6/01/2023 00:02:59
- 6/11/2023 00:02:32
