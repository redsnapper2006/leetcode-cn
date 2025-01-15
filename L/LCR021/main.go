package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	pFast := head
	for i := 0; i < n; i++ {
		pFast = pFast.Next
	}
	NH := ListNode{Val: 0, Next: head}
	pp := &NH

	for pFast != nil {
		pp = pp.Next
		pFast = pFast.Next
	}

	t := pp.Next.Next
	pp.Next = t
	return NH.Next
}

func main() {
	fmt.Println()
}
