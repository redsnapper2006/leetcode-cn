package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteMiddle(head *ListNode) *ListNode {
	p := head
	cnt := 0
	for p != nil {
		cnt++
		p = p.Next
	}
	steps := cnt / 2
	NH := ListNode{Val: 0, Next: head}
	pp := &NH
	p = head
	for steps > 0 {
		steps--
		pp = p
		p = p.Next
	}
	pp.Next = p.Next
	return NH.Next
}

func main() {
	fmt.Println()
}
