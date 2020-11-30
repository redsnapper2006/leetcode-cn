package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
	var pa, pb *ListNode
	p, nh := list1, list1

	for a >= 0 || b >= 0 {
		if a > 0 {
			pa = p
		}
		p = p.Next
		a--

		b--
		if b >= 0 {
			pb = p.Next
		}
	}
	pa.Next = list2
	p = list2
	for p.Next != nil {
		p = p.Next
	}
	p.Next = pb
	return nh
}
func main() {
	fmt.Println()
}
