package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func doubleIt(head *ListNode) *ListNode {
	var nh *ListNode = nil
	p := head
	for p != nil {
		t := p.Next
		p.Next = nh
		nh = p
		p = t
	}

	plus := false
	p = nh
	nh = nil
	for p != nil {
		p.Val *= 2
		if plus {
			p.Val++
		}
		plus = false
		if p.Val > 9 {
			plus = true
			p.Val -= 10
		}
		t := p.Next
		p.Next = nh

		nh = p
		p = t
	}

	if plus {
		return &ListNode{Val: 1, Next: nh}
	}
	return nh
}
