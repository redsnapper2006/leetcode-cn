package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeNodes(head *ListNode) *ListNode {
	pSlow, p := head, head.Next
	sum := 0
	for p != nil {
		if p.Val == 0 {
			pSlow.Val = sum
			if p.Next == nil {
				pSlow.Next = nil
			} else {
				pSlow = pSlow.Next
			}
			sum = 0
		} else {
			sum += p.Val
		}
		p = p.Next
	}
	return head
}

func main() {
	fmt.Println()
}
