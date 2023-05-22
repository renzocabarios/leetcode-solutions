## allow-one-function-call

## REFERENCE

- https://leetcode.com/problems/allow-one-function-call/

## SOLUTION

``` Typescript
function once<T extends (...args: any[]) => any>(fn: T): 
 ((...args: Parameters<T>) => ReturnType<T> | undefined) {
    let called = false;
    return function (...args) {
        if(called) return undefined;
        called = true;
        return fn(...args);
    };
}

/**
 * let fn = (a,b,c) => (a + b + c)
 * let onceFn = once(fn)
 *
 * onceFn(1,2,3); // 6
 * onceFn(2,3,6); // returns undefined without calling fn
 */
```

## TIMESTAMP

- 5/18/2023 00:08:31
- 5/19/2023 00:03:45
- 5/22/2023 00:01:34
