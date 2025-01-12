package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func reversePrint(head *ListNode) []int {
	p := head
	var r []int
	for p != nil {
		r = append(r, p.Val)
		p = p.Next
	}
	s, e :=0,len(r)-1
	for s < e {
		r[s],r[e] = r[e], r[s]
		s++
		e--
	}
	return r
}

func main() {
	fmt.Println("a")
}
