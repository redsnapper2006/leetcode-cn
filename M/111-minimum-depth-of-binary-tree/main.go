package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	left := minDepth(root.Left)
	right := minDepth(root.Right)

	if left == 0 && right == 0 {
		return 1
	}
	if left > 0 && right == 0 {
		return left + 1
	}
	if left == 0 && right > 0 {
		return right + 1
	}
	if left > 0 && right > 0 {
		if left > right {
			return right + 1
		} else {
			return left + 1
		}
	}
	return 0
}

func main() {
	o1, o2 := TreeNode{Val: 1, Left: nil, Right: nil}, TreeNode{Val: 2, Left: nil, Right: nil}
	o1.Left = &o2
	fmt.Println(minDepth(&o1))
}
