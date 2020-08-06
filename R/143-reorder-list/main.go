package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func reorderList(head *ListNode) {
	p := head
	for p.Next != nil {
		q := p
		var pp *ListNode
		for q.Next != nil {
			pp = q
			q = q.Next
		}
		t := p.Next
		p.Next = q
		q.Next = t
		pp.Next = nil
		p = t
	}
}

func main() {
	fmt.Println("a")
}
