package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func increasingBST(root *TreeNode) *TreeNode {
	Head := TreeNode{Val: 0}
	var left, right *TreeNode = nil, nil
	if root.Left != nil {
		left = increasingBST(root.Left)
	}
	if root.Right != nil {
		right = increasingBST(root.Right)
	}

	if left != nil {
		Head.Right = left
	}
	p := &Head
	for p.Right != nil {
		p = p.Right
	}
	root.Left = nil
	p.Right = root
	root.Right = right

	return Head.Right
}

func main() {
	fmt.Println("a")
}
