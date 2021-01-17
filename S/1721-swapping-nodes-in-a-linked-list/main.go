package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func swapNodes(head *ListNode, k int) *ListNode {
	pppp := head
	c := 0
	for pppp != nil {
		c++
		pppp = pppp.Next
	}
	if c%2 == 1 && k == (c+1)/2 {
		return head
	}
	if c%2 == 1 && k > (c+1)/2 {
		k = c - k + 1
	}
	if c%2 == 0 && k > c/2 {
		k = c - k + 1
	}
	NH := ListNode{Val: 0, Next: head}
	NNH := ListNode{Val: 0, Next: &NH}
	pp := &NH
	ppp := &NNH
	p := head
	for k > 1 {
		pp = p
		p = p.Next
		k--
	}
	pp1 := pp
	p1 := p

	pp = &NH
	for p != nil {
		ppp = ppp.Next
		pp = pp.Next
		p = p.Next
	}
	pp2 := ppp
	p2 := pp

	t1 := p1.Next
	t2 := p2.Next
	if t1 == p2 {
		p2.Next = p1
	} else {
		p2.Next = t1
	}

	p1.Next = t2
	pp1.Next = p2
	if t1 == p2 {
		p2.Next = p1
	} else {
		pp2.Next = p1
	}
	return NH.Next
}

func main() {
	fmt.Println()
}
