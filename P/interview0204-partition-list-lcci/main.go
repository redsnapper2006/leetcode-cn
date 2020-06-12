package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func partition(head *ListNode, x int) *ListNode {
	lt := ListNode{Val: 0, Next: nil}
	me := ListNode{Val: 0, Next: nil}
	plt, pme := &lt, &me

	p := head
	for p != nil {
		if p.Val < x {
			plt.Next = p
			plt = plt.Next
		} else {
			pme.Next = p
			pme = pme.Next
		}
		p = p.Next
	}
	pme.Next = nil
	plt.Next = me.Next

	return lt.Next
}

func main() {
	fmt.Println("a")
}
