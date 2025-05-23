package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func modifiedList(nums []int, head *ListNode) *ListNode {
	m := map[int]bool{}
	for _, n := range nums {
		m[n] = true
	}

	nh := &ListNode{Val: 0, Next: head}
	p := nh.Next
	pp := nh
	for p != nil {
		_, ok := m[p.Val]
		if ok {
			pp.Next = p.Next
		} else {
			pp = p
		}
		p = p.Next

	}
	return nh.Next
}
