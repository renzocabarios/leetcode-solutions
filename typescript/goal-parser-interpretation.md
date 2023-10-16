## goal-parser-interpretation

## REFERENCE

- https://leetcode.com/problems/goal-parser-interpretation/

## SOLUTION

``` Typescript
function interpret(command: string): string {
    return command.replace(/\(\)/g, 'o').replace(/\(al\)/g, 'al');
};
```

## TIMESTAMP

- 9/5/2023 00:5:19
- 9/6/2023 00:2:49
- 09/07/2023 00:02:04
- 09/12/2023 00:02:12
- 09/13/2023 00:01:26
- 09/18/2023 00:05:56
- 09/21/2023 00:00:53
- 09/22/2023 00:02:33
- 09/25/2023 00:01:37
- 10/01/2023 00:01:13



