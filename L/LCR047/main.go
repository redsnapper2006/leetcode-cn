package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pruneTree(root *TreeNode) *TreeNode {
	var recur func(root *TreeNode) bool
	recur = func(root *TreeNode) bool {
		if root == nil {
			return true
		}
		left := recur(root.Left)
		right := recur(root.Right)
		if left && right && root.Val == 0 {
			return true
		}
		if left {
			root.Left = nil
		}
		if right {
			root.Right = nil
		}
		return false
	}
	ret := recur(root)
	if ret {
		return nil
	}
	return root
}

func main() {
	fmt.Println()
}
