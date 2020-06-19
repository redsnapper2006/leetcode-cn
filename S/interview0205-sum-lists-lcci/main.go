package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	head := ListNode{Val: 0, Next: nil}
	h := &head
	p1, p2 := l1, l2
	isOver := false
	for p1 != nil || p2 != nil || isOver {
		var t1, t2 int
		if p1 != nil {
			t1 = p1.Val
			p1 = p1.Next
		}
		if p2 != nil {
			t2 = p2.Val
			p2 = p2.Next
		}
		t := t1 + t2
		if isOver {
			t++
		}
		if t >= 10 {
			isOver = true
			t -= 10
		} else {
			isOver = false
		}
		h.Val = t
		if p1 != nil || p2 != nil || isOver {
			h.Next = &ListNode{Val: 0, Next: nil}
		} else {
			h.Next = nil
		}
		h = h.Next
	}

	return &head
}

func main() {
	fmt.Println("a")
}
