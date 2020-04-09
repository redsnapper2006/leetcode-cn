package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func partition(head *ListNode, x int) *ListNode {
	if head == nil {
		return nil
	}

	lt := ListNode{Val: 0, Next: nil}
	ge := ListNode{Val: 0, Next: nil}

	p := head
	plt := &lt
	pge := &ge
	for p != nil {
		if p.Val < x {
			plt.Next = p
			plt = p
		} else {
			pge.Next = p
			pge = p
		}
		p = p.Next
	}
	plt.Next = nil
	pge.Next = nil

	if lt.Next == nil {
		return ge.Next
	}
	p = &lt
	for p.Next != nil {
		p = p.Next
	}
	p.Next = ge.Next

	return lt.Next
}

func main() {
	fmt.Println("a")
}
