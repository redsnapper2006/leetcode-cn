package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {
	count := 0
	p := head
	for p != nil {
		count++
		p = p.Next
	}

	s := count / 2
	move := s
	if count%2 == 1 {
		move++
	}
	right := head
	for move > 0 {
		right = right.Next
		move--
	}

	pp := nil
	for right != nil {
		t := right.Next
		right.Next = pp
		pp = right
		right = t
	}

	p = head
	for pp != nil && p != nil {
		if pp.Val != p.Val {
			return false
		}
		pp = pp.Next
		p = p.Next
	}
	return true
}

func main() {
	fmt.Println()
}
