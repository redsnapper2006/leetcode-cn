package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func insertIntoMaxTree(root *TreeNode, val int) *TreeNode {
	if root == nil {
		return &TreeNode{Val: val}
	}

	NH := TreeNode{Val: 0, Left: root}
	if val > root.Val {
		return &TreeNode{Val: val, Left: root}
	}
	pp := &NH
	p := root
	for p != nil && p.Val > val {
		pp = p
		p = p.Right
	}
	n := TreeNode{Val: val, Left: p}
	pp.Right = &n
	return NH.Left
}

func main() {
	fmt.Println("a")
}
