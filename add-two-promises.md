## add-two-promises

## REFERENCE

- https://leetcode.com/problems/add-two-promises/

## SOLUTION

``` Typescript
async function addTwoPromises(promise1: Promise<number>, promise2: Promise<number>): Promise<number> {
    return new Promise(async function(resolve, reject) {
        resolve(await promise1 + await promise2);
    });
    
};

/**
 * addTwoPromises(Promise.resolve(2), Promise.resolve(2))
 *   .then(console.log); // 4
 */
```

## TIMESTAMP

- 6/08/2023 00:03:12