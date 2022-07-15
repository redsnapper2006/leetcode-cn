package main

type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func intersect(quadTree1 *Node, quadTree2 *Node) *Node {

	if quadTree1.IsLeaf && quadTree2.IsLeaf {
		return &Node{Val: quadTree1.Val || quadTree2.Val, IsLeaf: true}
	}

	if quadTree1.IsLeaf && !quadTree2.IsLeaf {
		if quadTree1.Val {
			return &Node{Val: quadTree1.Val, IsLeaf: true}
		}
		return quadTree2
	}

	if !quadTree1.IsLeaf && quadTree2.IsLeaf {
		if quadTree2.Val {
			return &Node{Val: quadTree2.Val, IsLeaf: true}
		}
		return quadTree1
	}

	tl := intersect(quadTree1.TopLeft, quadTree2.TopLeft)
	tr := intersect(quadTree1.TopRight, quadTree2.TopRight)
	bl := intersect(quadTree1.BottomLeft, quadTree2.BottomLeft)
	br := intersect(quadTree1.BottomRight, quadTree2.BottomRight)
	if tl.IsLeaf && tr.IsLeaf && bl.IsLeaf && br.IsLeaf && tl.Val == tr.Val && tr.Val == bl.Val && bl.Val == br.Val {
		return &Node{Val: tl.Val, IsLeaf: true}
	}

	return &Node{TopLeft: tl, TopRight: tr, BottomLeft: bl, BottomRight: br}
}
