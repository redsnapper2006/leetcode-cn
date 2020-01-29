package main

import (
	"fmt"
)

// by Merge_Sort

type ListNode struct {
	Val  int
	Next *ListNode
}

// Merge the two lists.  Both must be non-NULL.
func merge(left *ListNode, right *ListNode) *ListNode {
	var head, p1, p2 *ListNode
	if left.Val < right.Val {
		head = left
		p1 = left
		p2 = right
	} else {
		head = right
		p1 = right
		p2 = left
	}
	// Print(left, "####")
	// Print(right, "****")
	for p1 != nil && p2 != nil && p1.Next != nil {
		if p1.Next.Val < p2.Val {
			p1 = p1.Next
		} else {
			p := p1.Next
			p1.Next = p2
			p1 = p1.Next
			p2 = p
		}
	}
	if p1.Next == nil {
		p1.Next = p2
	}

	return head
}

func sortList(head *ListNode) *ListNode {
	p := head
	size := 0
	for p != nil {
		p = p.Next
		size++
	}
	if size <= 1 {
		return head
	}
	middle := size / 2
	var left, right *ListNode
	c := 0
	left = head

	p = head
	for p != nil {
		if c < middle-1 {
			p = p.Next
			c++
		} else {
			right = p.Next
			p.Next = nil
			break
		}
	}

	return merge(sortList(left), sortList(right))
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
	o1, o2, o3, o4 := ListNode{Val: 4, Next: nil}, ListNode{Val: 2, Next: nil}, ListNode{Val: 1, Next: nil}, ListNode{Val: 3, Next: nil}
	o1.Next = &o2
	o2.Next = &o3
	o3.Next = &o4
	Print(sortList(&o1), "@@@@")
}
