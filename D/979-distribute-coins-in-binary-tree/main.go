package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func distributeCoins(root *TreeNode) int {
	ret := 0
	var recur func(root *TreeNode) int
	recur = func(root *TreeNode) int {
		if root.Left == nil && root.Right == nil {
			d := root.Val - 1
			if d < 0 {
				d = -d
			}
			ret += d
			return root.Val - 1
		}

		left, right := 0, 0
		if root.Left != nil {
			left = recur(root.Left)
		}
		if root.Right != nil {
			right = recur(root.Right)
		}
		d := left + right + root.Val - 1
		if d < 0 {
			d = -d
		}
		ret += d
		return left + right + root.Val - 1
	}
	recur(root)
	return ret
}

func main() {
	fmt.Println()
}
