package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func pairSum(head *ListNode) int {
	cnt := 0

	p := head
	for p != nil {
		p = p.Next
		cnt++
	}
	buf := make([]int, cnt/2)
	s := 0
	p = head
	for s < cnt/2 {
		buf[s] = p.Val
		s++
		p = p.Next
	}

	s--
	for s >= 0 {
		buf[s] += p.Val
		p = p.Next
		s--
	}
	max := buf[0]
	for i := 1; i < cnt/2; i++ {
		if max < buf[i] {
			max = buf[i]
		}
	}
	return max
}
