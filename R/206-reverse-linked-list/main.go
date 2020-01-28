package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
  if head == nil {
    return nil
  }
	if head.Next == nil {
		return head
	}
	b := reverseList(head.Next)
  p:= b
  for  p.Next != nil {
    p = p.Next
  }
	p.Next = head
	head.Next = nil
	return b
}

func reverseListV2(head *ListNode) *ListNode {
	var b *ListNode
	p := head
	for p != nil {
		t := p.Next
		p.Next = b
		b = p
		p = t
	}
	return b
}

func main() {
	fmt.Println("a")
}
