package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return head
	}
	p := head
	cur := head

	for p != nil {
		curVal := p.Val
		p = p.Next
		if p != nil && p.Val != curVal {
			cur.Next = p
			cur = p
		}
	}
	cur.Next = nil
	return head
}

func PrintList(p *ListNode) {
	for p != nil {
		fmt.Println(p.Val)
		p = p.Next
	}
}

func main() {
	o1, o2, o3, o4, o5 := ListNode{Val: 1, Next: nil}, ListNode{Val: 2, Next: nil}, ListNode{Val: 3, Next: nil}, ListNode{Val: 4, Next: nil}, ListNode{Val: 4, Next: nil}
	o1.Next = &o2
	o2.Next = &o3
	o3.Next = &o4
	o4.Next = &o5
	PrintList((deleteDuplicates(&o1)))
}
