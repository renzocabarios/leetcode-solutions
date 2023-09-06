## number-of-employees-who-met-the-target

## REFERENCE

- https://leetcode.com/problems/number-of-employees-who-met-the-target

## SOLUTION

``` Typescript
function numberOfEmployeesWhoMetTarget(hours: number[], target: number): number {
    let employees = 0;

    hours.forEach((e: any) => {
        if(e >= target) employees++;
    })

    return employees
};
```

## TIMESTAMP

- 9/2/2023 00:00:00
- 9/3/2023 00:01:20
- 9/6/2023 00:02:06
