package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func insertionSortList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	NEW := ListNode{Val: 0, Next: head}
	nh := head.Next
	if nh == nil {
		return head
	}
	head.Next = nil
	for nh != nil {
		p := NEW.Next
		pp := &NEW
		for p != nil && p.Val < nh.Val {
			pp = p
			p = p.Next
		}
		t := nh.Next
		// tt := pp.Next
		pp.Next = nh
		nh.Next = p
		nh = t
	}
	return NEW.Next
}

func main() {
	fmt.Println()
}
