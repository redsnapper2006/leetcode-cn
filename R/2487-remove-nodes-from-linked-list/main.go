package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNodes(head *ListNode) *ListNode {
	buf := []*ListNode{}

	p := head
	for p != nil {
		if len(buf) > 0 && buf[len(buf)-1].Val < p.Val {
			idx := len(buf) - 1
			for idx >= 0 && buf[idx].Val < p.Val {
				idx--
			}
			buf = buf[0 : idx+1]
		}
		buf = append(buf, p)
		p = p.Next
	}

	h := ListNode{Val: 0}
	p = &h
	for _, b := range buf {
		p.Next = b
		p = p.Next
	}
	return h.Next
}
