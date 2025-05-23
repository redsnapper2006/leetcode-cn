package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func reorderList(head *ListNode) {
	if head == nil {
		return
	}
	nodes := []*ListNode{}
	for node := head; node != nil; node = node.Next {
		nodes = append(nodes, node)
	}
	i, j := 0, len(nodes)-1
	for i < j {
		nodes[i].Next = nodes[j]
		i++
		if i == j {
			break
		}
		nodes[j].Next = nodes[i]
		j--
	}
	nodes[i].Next = nil
}

func reorderList2(head *ListNode) {
	if head == nil {
		return
	}

	p := head
	for p.Next != nil {
		q := p
		var pp *ListNode
		for q.Next != nil {
			pp = q
			q = q.Next
		}
		if pp == p {
			break
		}
		t := p.Next
		p.Next = q
		q.Next = t
		pp.Next = nil
		p = t
	}
}
