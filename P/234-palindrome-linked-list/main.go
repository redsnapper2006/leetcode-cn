package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {
	if head == nil {
		return true
	}

	p := head
	var buf []int
	for p != nil {
		buf = append(buf, p.Val)
		p = p.Next
	}
	if len(buf) == 1 {
		return true
	}

	var secHalf int
	if len(buf)%2 == 0 {
		secHalf = len(buf)/2 + 1
	} else {
		secHalf = len(buf)/2 + 2
	}
	for i, j := len(buf)/2-1, secHalf-1; i >= 0 && j < len(buf); i, j = i-1, j+1 {
		if buf[i] != buf[j] {
			return false
		}
	}

	return true
}

func main() {

	fmt.Println("a")
}
