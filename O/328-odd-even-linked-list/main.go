package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func oddEvenList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	var odd, even ListNode

	p1, p2 := &odd, &even
	c := 0
	p := head
	for p != nil {
		c++
		if c%2 == 1 {
			p1.Next = p
			p1 = p1.Next
		} else {
			p2.Next = p
			p2 = p2.Next
		}
		p = p.Next
	}
	p2.Next = nil
	p1.Next = even.Next
	return odd.Next
}

func Print(h *ListNode, title string) {
	fmt.Println(title)
	for h != nil {
		fmt.Println(h.Val)
		h = h.Next
	}
	fmt.Println(title)
}
func main() {
	o1, o2, o3, o4, o5 := ListNode{Val: 1, Next: nil}, ListNode{Val: 2, Next: nil}, ListNode{Val: 3, Next: nil}, ListNode{Val: 4, Next: nil}, ListNode{Val: 5, Next: nil}
	o1.Next = &o2
	o2.Next = &o3
	o3.Next = &o4
	o4.Next = &o5
	Print(oddEvenList(&o1), "@@@@")
}
