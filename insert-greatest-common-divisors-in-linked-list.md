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
    return !b ? a : gcd(b, a % b);
}

function insertGreatestCommonDivisors(head: ListNode | null): ListNode | null {
    if(head == null) return head;

    let current = head;

    while(current){
        let last = current;
        current = current.next;

        if(current !== null){
            let value = gcd(last.val, current.val);
            const newNode: ListNode = new ListNode(value);
            last.next = newNode;
            newNode.next = current;
        }
    }

    return head;
};
```


## TIMESTAMP

- 8/24/2023 00:16:48

