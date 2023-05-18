## CREATE HELLO WORLD FUNCTION

## REFERENCE

- https://leetcode.com/problems/create-hello-world-function/

## SOLUTION

``` javascript
function createHelloWorld() {
	return function(): string {
        return "Hello World"
    };
};

/**
 * const f = createHelloWorld();
 * f(); // "Hello World"
 */
```


## TIMESTAMP

- 5/17/2023 00:00:20 
- 5/18/2023 00:01:01 
