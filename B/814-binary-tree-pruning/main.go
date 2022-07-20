package main

import "fmt"

// TreeNode comments
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pruneTree(root *TreeNode) *TreeNode {
	var recur func(root *TreeNode) bool
	recur = func(root *TreeNode) bool {
		if root == nil {
			return false
		}
		left := recur(root.Left)
		right := recur(root.Right)
		if !left {
			root.Left = nil
		}
		if !right {
			root.Right = nil
		}
		return left || right || root.Val == 1
	}
	if !recur(root) {
		return nil
	}
	return root
}

func main() {
	fmt.Println("a")
}
