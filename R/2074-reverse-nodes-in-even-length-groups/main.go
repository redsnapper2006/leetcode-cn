package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseEvenLengthGroups(head *ListNode) *ListNode {
	steps := 0
	NH := ListNode{Val: 0, Next: head}
	p := head
	pp := &NH
	for p != nil {
		steps++
		remain := steps
		ppp := p
		for remain > 0 && p != nil {
			ppp = p
			p = p.Next
			remain--
		}

		if (steps-remain)%2 == 0 {
			t := pp.Next
			var tn *ListNode = nil
			var tpn *ListNode = nil
			for t != p {
				tpn = t.Next
				t.Next = tn
				tn = t
				t = tpn
			}
			pp.Next = tn
			for tn.Next != nil {
				tn = tn.Next
			}
			tn.Next = tpn
			pp = tn
		} else {
			pp = ppp
		}

	}
	return NH.Next
}
