package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func convertBST(root *TreeNode) *TreeNode {
	var recur func(root *TreeNode, p int) int
	recur = func(root *TreeNode, p int) int {
		if root == nil {
			return p
		}
		right := recur(root.Right, p)
		root.Val += right
		left := recur(root.Left, root.Val)
		return left
	}
	p := root
	recur(p, 0)
	return root
}

func main() {
	fmt.Println()
}
