package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func convertBiNode(root *TreeNode) *TreeNode {
	var recur func(root *TreeNode) *TreeNode
	recur = func(root *TreeNode) *TreeNode {
		if root == nil {
			return nil
		}

		left := recur(root.Left)
		right := recur(root.Right)

		TN := TreeNode{Val: 0}
		if left != nil {
			TN.Right = left
		}
		p := &TN
		for p.Right != nil {
			p = p.Right
		}
		p.Right = root
		root.Left = nil
		root.Right = right

		return TN.Right
	}
	return recur(root)
}

func main() {
	fmt.Println("a")
}
