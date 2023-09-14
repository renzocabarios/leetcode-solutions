## merge-nodes-in-between-zeros

## REFERENCE

- https://leetcode.com/problems/merge-nodes-in-between-zeros

## SOLUTION

``` Typescript
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

function mergeNodes(head: ListNode | null): ListNode | null {
    const dummy = new ListNode();
    let cur = dummy;
    let sum: number = 0;
    while(head){
        if (head.val === 0 && sum !== 0) {
            cur.next = new ListNode(sum);
            cur = cur.next;
            sum = 0;
        }
        sum += head.val;
        head = head.next;
    }

    return dummy.next ;
};
```

## TIMESTAMP

- 09/14/2023 00:07:56

