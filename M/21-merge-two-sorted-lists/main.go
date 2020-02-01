package main

import (
	"fmt"
)

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

func Print(node *ListNode) {
	for node != nil {
		fmt.Println(node.Val)
		node = node.Next
	}
}

func main() {
	o11, o12, o13 := ListNode{Val: 1, Next: nil}, ListNode{Val: 2, Next: nil}, ListNode{Val: 4, Next: nil}
	o21, o22, o23 := ListNode{Val: 1, Next: nil}, ListNode{Val: 3, Next: nil}, ListNode{Val: 4, Next: nil}

	o11.Next = &o12
	o12.Next = &o13
	o21.Next = &o22
	o22.Next = &o23

	Print(mergeTwoLists(&o11, &o21))
}
