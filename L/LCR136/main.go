package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteNode(head *ListNode, val int) *ListNode {
	if head.Val == val {
		return head.Next
	}
	parent, p := head, head.Next
	for p.Val != val {
		parent = p
		p = p.Next
	}
	parent.Next = p.Next
	return head
}

func main() {
	fmt.Println("a")
}
