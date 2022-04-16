package main

type Node struct {
	Val    int
	Left   *Node
	Right  *Node
	Parent *Node
}

func lowestCommonAncestor(p *Node, q *Node) *Node {
	m := map[int]*Node{}

	t := p
	for t != nil {
		m[t.Val] = t
		t = t.Parent
	}
	t = q
	for t != nil {
		_, ok := m[t.Val]
		if ok {
			return t
		}
		t = t.Parent
	}
	return nil
}
