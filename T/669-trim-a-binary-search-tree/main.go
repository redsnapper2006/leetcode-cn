package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func trimBST(root *TreeNode, L int, R int) *TreeNode {
	var recur func(root *TreeNode, L, R int) *TreeNode
	recur = func(root *TreeNode, L, R int) *TreeNode {
		if root.Left == nil && root.Right == nil {
			if root.Val < L || root.Val > R {
				return nil
			}
			return root
		}
		if root.Val < L {
			if root.Right != nil {
				root.Right = recur(root.Right, L, R)
			}
			return root.Right
		}
		if root.Val > R {
			if root.Left != nil {
				root.Left = recur(root.Left, L, R)
			}
			return root.Left
		}
		if root.Left != nil {
			root.Left = recur(root.Left, L, root.Val)
		}
		if root.Right != nil {
			root.Right = recur(root.Right, root.Val, R)
		}
		return root
	}
	return recur(root, L, R)
}

func main() {
	fmt.Println()
}
