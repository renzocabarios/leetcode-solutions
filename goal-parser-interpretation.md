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

- 9/5/2023 00:00:14