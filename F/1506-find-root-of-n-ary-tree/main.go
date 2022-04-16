package main

type Node struct {
	Val      int
	Children []*Node
}

func findRoot(tree []*Node) *Node {
	m := map[*Node]*Node{}
	for _, t := range tree {
		for _, c := range t.Children {
			m[c] = t
		}
	}
	t := tree[0]

	for {
		_, ok := m[t]
		if !ok {
			break
		}
		t = m[t]
	}
	return t
}
