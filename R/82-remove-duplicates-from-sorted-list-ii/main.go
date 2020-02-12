package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	var newHead, q *ListNode
	p := head

	c := 0
	for p.Next != nil {
		c++
		if p.Val != p.Next.Val {
			if c == 1 {
				if newHead == nil {
					newHead = p
					q = p
				} else {
					q.Next = p
					q = p
				}
			}
			c = 0
		}
		p = p.Next
	}

	if c > 0 {
		if newHead == nil {
			return nil
		} else {
			q.Next = nil
		}
	} else {
		if newHead == nil {
			newHead = p
		} else {
			q.Next = p
		}
	}
	return newHead
}

func main() {
	fmt.Println("a")
}
