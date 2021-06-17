package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func longestUnivaluePath(root *TreeNode) int {
	var recur func(root *TreeNode) (int, int)
	recur = func(root *TreeNode) (int, int) {
		if (root == nil) || (root.Left == nil && root.Right == nil) {
			return 0, 0
		}

		depthLeft, depthRight := 0, 0
		pathLeft, pathRight := 0, 0
		if root.Left != nil {
			depthLeft, pathLeft = recur(root.Left)
			if root.Left.Val == root.Val {
				depthLeft++
			}
		}
		if root.Right != nil {
			depthRight, pathRight = recur(root.Right)
			if root.Right.Val == root.Val {
				depthRight++
			}
		}

		depth := 0
		if root.Left != nil && root.Left.Val == root.Val {
			depth = depthLeft
		}
		if root.Right != nil && root.Right.Val == root.Val {
			if depth < depthRight {
				depth = depthRight
			}
		}

		path := 0
		if root.Left != nil && root.Left.Val == root.Val {
			path += depthLeft
		}
		if root.Right != nil && root.Right.Val == root.Val {
			path += depthRight
		}
		if path < pathLeft {
			path = pathLeft
		}
		if path < pathRight {
			path = pathRight
		}

		return depth, path
	}
	_, r := recur(root)
	return r
}

func main() {
	fmt.Println("a")
}
