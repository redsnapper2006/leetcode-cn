package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func hasPathSum(root *TreeNode, sum int) bool {
	if root == nil {
		return false
	}
	if root.Left == nil && root.Right == nil {
		if sum == root.Val {
			return true
		} else {
			return false
		}
	}

	left, right := false, false
	if root.Left != nil {
		left = hasPathSum(root.Left, sum-root.Val)
	}
	if root.Right != nil {
		right = hasPathSum(root.Right, sum-root.Val)
	}
	return left || right
}

func main() {
	fmt.Println("a")
}
