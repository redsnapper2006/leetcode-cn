package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(r *TreeNode) bool {
	if r == nil {
		return true
	}

	if r.Left != nil && r.Left.Val >= r.Val {
		return false
	}
	if r.Right != nil && r.Right.Val <= r.Val {
		return false
	}

	var recur func(r *TreeNode, min, max int) bool
	recur = func(r *TreeNode, min, max int) bool {
		if r == nil {
			return true
		}

		if r.Left != nil {
			if r.Left.Val >= r.Val {
				return false
			}
			if r.Left.Val <= min || r.Left.Val >= max {
				return false
			}
		}

		if r.Right != nil {
			if r.Right.Val <= r.Val {
				return false
			}
			if r.Right.Val >= max || r.Right.Val <= min {
				return false
			}
		}

		return recur(r.Left, min, r.Val) && recur(r.Right, r.Val, max)
	}
	return recur(r.Left, -1<<63, r.Val) && recur(r.Right, r.Val, 1<<63-1)
}

func main() {
	fmt.Println("a")
}
