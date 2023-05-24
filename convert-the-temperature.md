## convert-the-temperature

## REFERENCE

- https://leetcode.com/problems/convert-the-temperature/

## SOLUTION

``` typescript
function convertTemperature(celsius: number): number[] {
    let ans = [celsius + 273.15, (celsius * 1.80+32)]
    return ans;
};
```


## TIMESTAMP

- 5/24/2023 00:02:15
