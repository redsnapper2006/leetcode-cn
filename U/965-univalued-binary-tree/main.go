package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isUnivalTree(root *TreeNode) bool {
	if root == nil {
		return true
	}

	left := true
	if root.Left != nil {
		if root.Val != root.Left.Val {
			return false
		}
		left = isUnivalTree(root.Left)
	}
	right := true
	if root.Right != nil {
		if root.Val != root.Right.Val {
			return false
		}
		right = isUnivalTree(root.Right)
	}
	return left && right
}

func main() {
	fmt.Println("a")
}
