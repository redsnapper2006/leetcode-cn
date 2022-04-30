package main

type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func construct(grid [][]int) *Node {
	right, bottom := len(grid), len(grid[0])

	var recur func(grid [][]int, left, top, right, bottom int) *Node
	recur = func(grid [][]int, left, top, right, bottom int) *Node {
		if left+1 == right {
			return &Node{Val: grid[left][top] == 1, IsLeaf: true}
		}

		lt := recur(grid, left, top, (right+left)/2, (bottom+top)/2)
		rt := recur(grid, left, (bottom+top)/2, (right+left)/2, bottom)
		lb := recur(grid, (right+left)/2, top, right, (bottom+top)/2)
		rb := recur(grid, (right+left)/2, (bottom+top)/2, right, bottom)

		if lt.Val == rt.Val && rt.Val == lb.Val && lb.Val == rb.Val && lt.IsLeaf && rt.IsLeaf && lb.IsLeaf && rb.IsLeaf {
			return &Node{Val: lt.Val, IsLeaf: true}
		}
		return &Node{TopLeft: lt, TopRight: rt, BottomLeft: lb, BottomRight: rb}
	}

	return recur(grid, 0, 0, right, bottom)
}
