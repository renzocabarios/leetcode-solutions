## array-wrapper

## REFERENCE

- https://leetcode.com/problems/array-wrapper/

## SOLUTION

``` typescript
class ArrayWrapper {
    private nums: number[];
	constructor(nums: number[]) {
        this.nums = nums;
    }

	valueOf() {
        return this.nums.reduce((acc, curr) => acc+ curr, 0)
    }

	toString() {
        return `[${String(this.nums.toString())}]`
    }
};

/**
 * const obj1 = new ArrayWrapper([1,2]);
 * const obj2 = new ArrayWrapper([3,4]);
 * obj1 + obj2; // 10
 * String(obj1); // "[1,2]"
 * String(obj2); // "[3,4]"
 */
```


## TIMESTAMP

- 6/02/2023 00:02:52
- 6/03/2023 00:01:51
- 6/06/2023 00:01:35
- 6/11/2023 00:01:28
- 7/24/2023 00:01:03
- 10/05/2023 00:02:027
