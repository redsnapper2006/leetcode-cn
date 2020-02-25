package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeElements(head *ListNode, val int) *ListNode {
	p := ListNode{Val: val + 1, Next: head}
	q := &p
	prev := &p
	for q != nil {
		if q.Val == val {
			prev.Next = q.Next
		} else {
			prev = q
		}
		q = q.Next
	}
	return p.Next
}

func main() {
	fmt.Println("a")
}
