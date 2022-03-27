package main

type Node struct {
	Val  int
	Next *Node
}

func insert(aNode *Node, x int) *Node {
	if aNode == nil {
		p := Node{Val: x}
		p.Next = &p
		return &p
	}

	if aNode != nil && aNode.Next == aNode {
		p := Node{Val: x, Next: aNode}
		aNode.Next = &p
		return aNode
	}

	pp, p := aNode, aNode.Next
	for p != aNode {
		if pp.Val <= x && p.Val > x {
			break
		}
		pp = p
		p = p.Next
	}
	if pp.Val <= x && p.Val > x {
		pp.Next = &Node{Val: x, Next: p}
	} else {

		pp, p := aNode, aNode.Next
		for pp.Val <= p.Val {
			pp = p
			p = p.Next
			if pp == aNode {
				break
			}
		}

		pp.Next = &Node{Val: x, Next: p}
	}

	return aNode
}
