## add-two-promises

## REFERENCE

- https://leetcode.com/problems/add-two-promises/

## SOLUTION

``` Typescript
async function addTwoPromises(promise1: Promise<number>, promise2: Promise<number>): Promise<number> {
    return Promise.all([promise1,promise2]).then((_) => {
        return _.reduce((acc, curr) => acc+curr, 0)
    })
};

/**
 * addTwoPromises(Promise.resolve(2), Promise.resolve(2))
 *   .then(console.log); // 4
 */
```

## TIMESTAMP

- 6/08/2023 00:03:12
- 6/10/23 00:01:40
- 6/13/23 00:00:57
- 7/20/23 00:02:17
- 08/12/23 00:01:30
