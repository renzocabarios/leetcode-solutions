## maximum-number-of-words-found-in-sentences

## REFERENCE

- https://leetcode.com/problems/maximum-number-of-words-found-in-sentences

## SOLUTION

``` Typescript
function mostWordsFound(sentences: string[]): number {
    let max: number = 0;

    sentences.forEach((sentence: string) => {
        max = Math.max(max, sentence.split(" ").length)
    })

    return max;
};
```

## TIMESTAMP

- 09/06/2023 00:03:09
- 09/07/2023 00:01:54
- 09/12/2023 00:01:36
- 09/22/2023 00:03:26







