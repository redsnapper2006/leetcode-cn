package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func getIntersectionNode(headA, headB *ListNode) *ListNode {
	if headA == nil || headB == nil {
		return nil
	}

	pA, pB := headA, headB
	for pA != pB {
		if pA == nil {
			pA = headB
		} else {
			pA = pA.Next
		}
		if pB == nil {
			pB = headA
		} else {
			pB = pB.Next
		}
	}
	return pA
}

func getIntersectionNodeV2(headA, headB *ListNode) *ListNode {
	pA, pB := headA, headB
	cA, cB := 0, 0
	for pA != nil {
		cA++
		pA = pA.Next
	}
	for pB != nil {
		cB++
		pB = pB.Next
	}
	fmt.Println(cA, cB)
	pA, pB = headA, headB
	for i := 0; i < cA*cB; i++ {
		if pA == nil {
			pA = headA
		}
		if pB == nil {
			pB = headB
		}
		if pA != nil && pB != nil && pA == pB {
			return pA
		}

		pA = pA.Next
		pB = pB.Next
	}
	return nil
}

func main() {
	fmt.Println("a")
}
