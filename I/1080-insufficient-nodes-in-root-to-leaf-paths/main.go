package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sufficientSubset(root *TreeNode, limit int) *TreeNode {
	HEAD := TreeNode{Val: 0, Left: root}

	var recur func(root *TreeNode, limit int, sum int) bool
	recur = func(root *TreeNode, limit int, sum int) bool {

		v := sum + root.Val
		if root.Left == nil && root.Right == nil {
			return v >= limit
		}

		left, right := false, false
		if root.Left != nil {
			left = recur(root.Left, limit, v)
		}
		if !left {
			root.Left = nil
		}
		if root.Right != nil {
			right = recur(root.Right, limit, v)
		}
		if !right {
			root.Right = nil
		}
		return left || right
	}

	recur(&HEAD, limit, 0)
	return HEAD.Left
}
