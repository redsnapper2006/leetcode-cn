package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func sortList(head *ListNode) *ListNode {
	pSlow, pFast := head, head
	pp := head
	for pFast != nil && pFast.Next != nil {
		pFast = pFast.Next
		if pFast != nil {
			pFast = pFast.Next
		}
		pp = pSlow
		pSlow = pSlow.Next
	}
	if pSlow == pFast {
		return pSlow
	}
	pp.Next = nil
	h := sortList(head)
	t := sortList(pSlow)
	nh := ListNode{Val: 0}
	pnh := &nh
	for h != nil && t != nil {
		if h.Val > t.Val {
			pnh.Next = t
			pnh = t
			t = t.Next
		} else {
			pnh.Next = h
			pnh = h
			h = h.Next
		}
	}
	if h != nil {
		pnh.Next = h
	}
	if t != nil {
		pnh.Next = t
	}
	return nh.Next
}

func main() {
	fmt.Println()
}
