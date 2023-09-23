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
    const main: ListNode = new ListNode();
    let temp: ListNode = main;
    let sum: number = 0;

    while(head){
        if(head.val === 0 && sum !== 0){
            temp.next = new ListNode(sum);
            temp = temp.next;
            sum = 0;
        }
        sum += head.val;
        head = head.next;
    }
    return main.next;
};
```

## TIMESTAMP

- 09/14/2023 00:07:56
- 09/22/2023 00:12:57
- 09/23/2023 00:10:12










