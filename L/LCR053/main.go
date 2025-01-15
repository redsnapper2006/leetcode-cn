package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderSuccessor(root *TreeNode, p *TreeNode) *TreeNode {
	if p.Right != nil {
		h := p.Right
		parent := h
		for h != nil {
			parent = h
			h = h.Left
		}
		return parent
	}

	var parent *TreeNode = nil
	h := root

	for h != nil && h != p {
		if h.Val > p.Val {
			parent = h
			h = h.Left
		} else if h.Val < p.Val {
			h = h.Right
		}
	}
	return parent
}

func main() {
	fmt.Println()
}
