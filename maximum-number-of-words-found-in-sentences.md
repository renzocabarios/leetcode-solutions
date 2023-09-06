## maximum-number-of-words-found-in-sentences

## REFERENCE

- https://leetcode.com/problems/maximum-number-of-words-found-in-sentences

## SOLUTION

``` Typescript
function mostWordsFound(sentences: string[]): number {
    let max: number = 0;

    sentences.forEach((e:string) => {
        const length = e.split(" ").length
        if(max < length) max = length
    })

    return max;
};
```

## TIMESTAMP

- 09/06/2023 00:03:09

