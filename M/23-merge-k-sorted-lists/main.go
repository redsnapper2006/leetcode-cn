package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeKListsV2(lists []*ListNode) *ListNode {
	p := make([]*ListNode, len(lists))
	for i := 0; i < len(lists); i++ {
		p[i] = lists[i]
	}

	isEndLoop := func(p []*ListNode) bool {
		for i := 0; i < len(p); i++ {
			if p[i] != nil {
				return false
			}
		}
		return true
	}
	getMin := func(p []*ListNode) int {
		isFirst := true
		t := -1
		for i := 0; i < len(p); i++ {
			if p[i] != nil {
				if isFirst {
					t = i
					isFirst = false
				} else {
					if p[i].Val < p[t].Val {
						t = i
					}
				}
			}
		}
		return t
	}

	head := ListNode{Val: 0, Next: nil}
	h := &head
	for {
		if isEndLoop(p) {
			break
		}

		t := getMin(p)
		h.Next = p[t]
		p[t] = p[t].Next
		h = h.Next
	}
	return head.Next
}

func mergeKLists(lists []*ListNode) *ListNode {
	if lists == nil || len(lists) == 0 {
		return nil
	}

	merge2Lists := func(p1, p2 *ListNode) *ListNode {
		head := ListNode{Val: 0, Next: nil}
		h := &head
		for p1 != nil && p2 != nil {
			if p1.Val > p2.Val {
				h.Next = p2
				p2 = p2.Next
			} else {
				h.Next = p1
				p1 = p1.Next
			}
			h=h.Next
		}
		if p1 == nil {
			h.Next = p2
		}
		if p2 == nil {
			h.Next = p1
		}
		return head.Next
	}
	for len(lists) > 1 {
		l, r := 0, len(lists)-1
		var t []*ListNode
		for l < r {
			t = append(t, merge2Lists(lists[l], lists[r]))
			l++
			r--
		}
		if l == r {
			t = append(t, lists[l])
		}
		lists = t
	}

	return lists[0]
}

func main() {
	fmt.Println("a")
}
