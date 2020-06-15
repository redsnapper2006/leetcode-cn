package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeDuplicateNodes(head *ListNode) *ListNode {
	NH := ListNode{Val: 0}
	slow := &NH
	m := map[int]int{}
	p := head
	for p != nil {
		_, ok := m[p.Val]
		if !ok {
			slow.Next = p
			slow = slow.Next
			m[p.Val]++
		}
		p = p.Next
	}
	slow.Next = nil
	return NH.Next
}

func main() {
	fmt.Println("a")
}
