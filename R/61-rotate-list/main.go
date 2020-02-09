package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func rotateRight(head *ListNode, k int) *ListNode {
	if head == nil || k == 0 {
		return head
	}
	p := head
	var cur *ListNode
	cur = nil
	len := 0
	for p != nil {
		t := p.Next
		p.Next = cur
		cur = p
		p = t
		len++
	}

	k %= len
	if k == 0 {
		k += len
	}

	times := 0
	var newHead, secondHead, firstEnd *ListNode
	newHead, secondHead, firstEnd = nil, nil, cur

	p = cur
	cur = nil
	for p != nil && times < k {
		secondHead = p.Next
		p.Next = cur
		cur = p
		p = secondHead
		times++
	}
	newHead = cur

	p = secondHead
	cur = nil
	for p != nil {
		t := p.Next
		p.Next = cur
		cur = p
		p = t
	}
	firstEnd.Next = cur
	return newHead
}

func PrintList(p *ListNode) {
	for p != nil {
		fmt.Println(p.Val)
		p = p.Next
	}
}

func main() {
	// o1, o2, o3, o4, o5 := ListNode{Val: 1, Next: nil}, ListNode{Val: 2, Next: nil}, ListNode{Val: 3, Next: nil}, ListNode{Val: 4, Next: nil}, ListNode{Val: 5, Next: nil}
	// o1.Next = &o2
	// o2.Next = &o3
	// o3.Next = &o4
	// o4.Next = &o5

	o1 := ListNode{Val: 1, Next: nil}
	PrintList(rotateRight(&o1, 1))
}
