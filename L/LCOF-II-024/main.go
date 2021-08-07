package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var b *ListNode
	p := head
	for p != nil {
		t := p.Next
		p.Next = b
		b = p
		p = t
	}
	return b
}

func main() {
	fmt.Println()
}
