## function-composition

## REFERENCE

- https://leetcode.com/problems/function-composition/

## SOLUTION

``` Typescript
type F = (x: number) => number;

function compose(functions: F[]): F {
	return function(x: number): number {
        return functions.reduceRight((acc, curr) => curr(acc), x)
    }
};

/**
 * const fn = compose([x => x + 1, x => 2 * x])
 * fn(4) // 9
 */
```

## TIMESTAMP

- 5/18/2023 00:10:30
- 5/19/2023 00:01:30
