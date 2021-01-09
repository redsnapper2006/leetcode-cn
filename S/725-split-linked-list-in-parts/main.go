package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func splitListToParts(root *ListNode, k int) []*ListNode {
	p := root
	cnt := 0
	for p != nil {
		p = p.Next
		cnt++
	}

	m := cnt % k
	c := cnt / k
	if m > 0 {
		c++
	}
	p = root
	var ret []*ListNode
	times := 0
	for p != nil || times < k {
		ret = append(ret, p)
		t := c
		pp := p
		for ; t > 0; t-- {
			pp = p
			p = p.Next
		}
		if pp != nil {
			pp.Next = nil
		}
		m--
		if m == 0 {
			c--
		}
		times++
	}
	return ret
}

func main() {
	fmt.Println()
}
