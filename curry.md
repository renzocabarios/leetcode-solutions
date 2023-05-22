## curry

## REFERENCE

- https://leetcode.com/problems/curry/

## SOLUTION

``` Typescript
function curry(fn: Function): Function {
    return function curried(...args): Function {
        if(args.length === fn.length) return fn(...args);
        return function (...newArgs): Function{
            return curried(...args, ...newArgs);
        }
    };
};

/**
 * function sum(a, b) { return a + b; }
 * const csum = curry(sum);
 * csum(1)(2) // 3
 */
```

## TIMESTAMP

- 5/18/2023 00:42:58
- 5/19/2023 00:04:56
- 5/22/2023 00:02:32
