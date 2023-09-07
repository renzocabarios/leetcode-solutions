## subtract-the-product-and-sum-of-digits-of-an-integer

## REFERENCE

- https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

## SOLUTION

``` Typescript
function subtractProductAndSum(n: number): number {
    const numbers: number[] = Array.from(String(n), (e: string) => Number(e))

    let product: number = 1;
    let sum: number = 0;

    numbers.forEach((e: number) => {
        product *= e;
        sum += e;
    })

    return product - sum
};
```

## TIMESTAMP

- 09/07/2023 00:05:41

