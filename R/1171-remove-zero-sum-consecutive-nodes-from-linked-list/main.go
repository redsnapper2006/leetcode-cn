package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeZeroSumSublists(head *ListNode) *ListNode {
	NH := ListNode{Val: 0, Next: head}
	M := map[int]*ListNode{}
	M[0] = &NH
	p := head
	sum := 0
	for p != nil {
		sum += p.Val
		v, ok := M[sum]
		if !ok {
			M[sum] = p
		} else {
			if p.Val != 0 {
				q := v.Next
				oldsum := sum
				for q != p {
					oldsum += q.Val
					if oldsum != 0 {
						delete(M, oldsum)
					}
					q = q.Next
				}
			}
			v.Next = p.Next
		}
		p = p.Next
	}
	return NH.Next
}

func main() {
	fmt.Println("a")
}
