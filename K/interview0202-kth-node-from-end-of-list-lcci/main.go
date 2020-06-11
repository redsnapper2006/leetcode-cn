package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func kthToLast(head *ListNode, k int) int {
	f, s := head, head
	for i := 1; i < k; i++ {
		f = f.Next
	}
	for f != nil {
		f = f.Next
		s = s.Next
	}
	return s.Val
}

func main() {
	fmt.Println("a")
}
