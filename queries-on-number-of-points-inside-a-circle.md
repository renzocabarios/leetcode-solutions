## queries-on-number-of-points-inside-a-circle

## REFERENCE

- https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
- x^2 + y^ <= r^2

## SOLUTION

``` Typescript
function countPoints(points: number[][], queries: number[][]): number[] {
    const answer: number[] = [];

    for(let i: number = 0; queries.length > i; i++){
        let count: number = 0;
        for(let j: number = 0; points.length > j; j++){
            const x2 = Math.pow(queries[i][0] - points[j][0], 2);
            const y2 = Math.pow(queries[i][1] - points[j][1], 2);
            const r2 = Math.pow(queries[i][2], 2);
            if(x2 + y2 <= r2){
                count++;
            }
        }

        answer.push(count)
    }
    return answer;
};


```

## TIMESTAMP

- 09/12/2023 00:33:37
- 09/13/2023 00:06:57
- 09/18/2023 00:08:57
- 09/21/2023 00:03:22
- 09/22/2023 00:03:23



















