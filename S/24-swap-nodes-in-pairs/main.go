package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func swapPairs(head *ListNode) *ListNode {
	if head == nil || (head != nil && head.Next == nil) {
		return head
	}

	t := ListNode{Val: 0, Next: head}
	h := &t

	for {
		p1 := h.Next
		var p2 *ListNode
		if p1 != nil {
			p2 = h.Next.Next
		}
		if p1 != nil && p2 != nil {
			t := p2.Next
			h.Next = p2
			p2.Next = p1
			p1.Next = t
			h = p1
		} else {
			break
		}
	}
	return t.Next
}

func main() {
	fmt.Println("a")
}
