package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseBetween(head *ListNode, m int, n int) *ListNode {
	if m == n {
		return head
	}
	NH := ListNode{Val: 0, Next: head}
	p := &NH

	for i := 0; i < m-1; i++ {
		p = p.Next
	}
	s := p.Next
	e := p.Next
	var next *ListNode = nil
	var t *ListNode = nil
	for i := 0; i <= n-m; i++ {
		t = s.Next
		s.Next = next
		next = s
		s = t
	}
	p.Next = next
	e.Next = t

	return NH.Next
}

func main() {
	fmt.Println("a")
}
