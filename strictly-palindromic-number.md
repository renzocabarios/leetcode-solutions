## strictly-palindromic-number

## REFERENCE

- https://leetcode.com/problems/strictly-palindromic-number/

## SOLUTION

``` Typescript
function isStrictlyPalindromic(n: number): boolean {
    return false;
};

```

``` Typescript
function isStrictlyPalindromic(n: number): boolean {
    for(let i = 2; i < n - 2; i++){
        const based = n.toString(i);
        if(!isPalindrome(based)) return false;
    }

    return true;
};

function isPalindrome(number: string): boolean {
    let left = 0;
    let right = number.length -1;

    while(left < right){
        if(number.charAt(left) !== number.charAt(right)) return false
        left++;
        right--;
    }

    return true;
};

```

## TIMESTAMP

- 9/3/2023 00:19:36

