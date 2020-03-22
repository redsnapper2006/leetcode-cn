package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func middleNodeV2(head *ListNode) *ListNode {
	p := head
	c := 0
	for p != nil {
		p = p.Next
		c++
	}
	steps := c / 2
	p = head
	for steps > 0 {
		p = p.Next
		steps--
	}
	return p
}

func middleNode(head *ListNode) *ListNode {
	fast, slow := head, head
	for fast.Next != nil {
		slow = slow.Next
		fast = fast.Next
		if fast.Next != nil {
			fast = fast.Next
		}
	}
	return slow
}

func main() {
	fmt.Println("a")
}
