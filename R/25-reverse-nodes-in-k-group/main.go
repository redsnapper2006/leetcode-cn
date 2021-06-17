package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseKGroup(head *ListNode, k int) *ListNode {
	NEWHEAD := ListNode{Val: 0, Next: head}

	pp := &NEWHEAD
	for {
		p := pp
		NewEnd := p.Next
		i := 0
		for p != nil && i < k {
			p = p.Next
			i++
		}
		if p == nil {
			break
		}
		NewStart := p
		nNext := p.Next
		p = NewEnd
		for p != NewStart {
			t := p.Next
			p.Next = nNext
			nNext = p
			p = t
		}
		p.Next = nNext
		pp.Next = NewStart
		pp = NewEnd
	}
	return NEWHEAD.Next
}

func main() {
	fmt.Println("a")
}
