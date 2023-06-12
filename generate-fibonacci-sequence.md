## generate-fibonacci-sequence

## REFERENCE

- https://leetcode.com/problems/generate-fibonacci-sequence/

## SOLUTION

``` typescript
function* fibGenerator(): Generator<number, any, number> {
    let n1 = 0, n2 = 1;

    while(true){
        yield n1;
        [n1, n2] = [n2, n1+n2]
    }
};

/**
 * const gen = fibGenerator();
 * gen.next().value; // 0
 * gen.next().value; // 1
 */
```


## TIMESTAMP

- 6/04/2023 00:11:02
- 6/05/2023 00:02:14
- 6/08/2023 00:01:38
- 6/13/2023 00:01:15
