## insert-greatest-common-divisors-in-linked-list

## REFERENCE

- https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/

## SOLUTION

``` javascript
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function gcd(a: number, b: number): number {
    return !b ? a : gcd(b, a % b)
};

function insertGreatestCommonDivisors(head: ListNode | null): ListNode | null {

    if(!head) return head;
    let current: ListNode = head;

    while(current.next !== null){
        let back = current;
        current = current.next;
        const value: number = gcd(back.val, current.val);
        back.next = new ListNode(value, current)
    }

    return head;
};
```


## TIMESTAMP

- 8/24/2023 00:16:48
- 8/25/2023 00:06:22
- 8/26/2023 00:08:06
- 8/27/2023 00:07:17
- 8/30/2023 00:08:23
- 9/4/2023 00:05:94
- 09/22/2023 00:05:30
- 09/23/2023 00:08:34
- 09/24/2023 00:03:55
- 09/25/2023 00:02:42
- 09/28/2023 00:04:22
- 10/05/2023 00:03:30
