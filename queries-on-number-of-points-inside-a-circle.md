## queries-on-number-of-points-inside-a-circle

## REFERENCE

- https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
- x^2 + y^ <= r^2

## SOLUTION

``` Typescript
function countPoints(points: number[][], queries: number[][]): number[] {
    const counts: number[] = []
    for(let i: number = 0; i < queries.length; i++){
        let count: number = 0;
        for(let j: number = 0; j < points.length; j++){
            const x2: number = Math.abs(Math.pow(queries[i][0] - points[j][0], 2));
            const y2: number = Math.abs(Math.pow(queries[i][1] - points[j][1], 2));
            const r2: number = Math.abs(Math.pow(queries[i][2], 2));
            if(x2 + y2 <= r2)  count++;
        }
        counts.push(count)
    }

    return counts;
};


```

## TIMESTAMP

- 09/12/2023 00:33:37
- 09/13/2023 00:06:57
- 09/18/2023 00:08:57
- 09/21/2023 00:03:22
- 09/22/2023 00:03:23
- 09/25/2023 00:03:51
- 10/02/2023 00:03:48

















