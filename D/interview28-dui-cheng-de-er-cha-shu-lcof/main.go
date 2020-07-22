package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isMirror(left, right *TreeNode) bool {
	if !((left == nil && right == nil) || (left != nil && right != nil && left.Val == right.Val)) {
		return false
	}
	if left == nil && right == nil {
		return true
	}
	return isMirror(left.Left, right.Right) && isMirror(left.Right, right.Left)
}

func isSymmetric(root *TreeNode) bool {
	if root == nil {
		return true
	}

	return isMirror(root.Left, root.Right)
}

func main() {
	fmt.Println("a")
}
