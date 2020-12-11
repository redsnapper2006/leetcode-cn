package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func removeLeafNodes(root *TreeNode, target int) *TreeNode {
	var recur func(root *TreeNode, target int) *TreeNode
	recur = func(root *TreeNode, target int) *TreeNode {
		if root.Left == nil && root.Right == nil {
			if root.Val == target {
				return nil
			}
			return root
		}
		if root.Left != nil {
			root.Left = recur(root.Left, target)
		}
		if root.Right != nil {
			root.Right = recur(root.Right, target)
		}
		if root.Left == nil && root.Right == nil && root.Val == target {
			return nil
		}
		return root
	}
	return recur(root, target)
}

func main() {
	fmt.Println()
}
