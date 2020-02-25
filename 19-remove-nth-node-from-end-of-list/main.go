package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	buf := make([]*ListNode, n+1)
	offset := 0
	p := head
	for p != nil {
		buf[offset] = p
		offset++
		offset %= (n + 1)
		p = p.Next
	}

	p1 := buf[offset]
	p2 := buf[(offset+2)%(n+1)]
	if p1 == nil {
		return p2
	}
	if p1 == p2 {
		p1.Next = nil
	} else {
		p1.Next = p2
	}
	return head
}

func main() {
	fmt.Println("a")
}
