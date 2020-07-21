package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	if l1 == nil {
		return l2
	}
	if l2 == nil {
		return l1
	}
	var p, p1, p2 *ListNode

	if l1.Val > l2.Val {
		p = l2
		p1 = l2
		p2 = l1
	} else {
		p = l1
		p1 = l1
		p2 = l2
	}

	for p1.Next != nil && p2 != nil {
		if p1.Next.Val > p2.Val {
			t := p1.Next
			p1.Next = p2
			p2 = t
		} else {
			p1 = p1.Next
		}
	}
	if p1.Next == nil && p2 != nil {
		p1.Next = p2
	}

	return p
}

func main() {
	fmt.Println("a")
}
