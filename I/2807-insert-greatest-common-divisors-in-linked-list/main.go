package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func insertGreatestCommonDivisors(head *ListNode) *ListNode {
	nh := head
	p := head

	var GCD func(x, y int) int
	GCD = func(x, y int) int {
		p1, p2 := x, y
		if p2 > p1 {
			p1, p2 = y, x
		}
		if p1%p2 == 0 {
			return p2
		}
		return GCD(p1%p2, p2)
	}
	for p != nil && p.Next != nil {
		t := p.Next
		p.Next = &ListNode{Val: GCD(p.Val, t.Val), Next: t}
		p = t
	}
	return nh
}
