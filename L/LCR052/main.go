package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func increasingBST(root *TreeNode) *TreeNode {
	var recur func(root *TreeNode) *TreeNode
	recur = func(root *TreeNode) *TreeNode {
		if root == nil {
			return nil
		}
		left := increasingBST(root.Left)
		right := increasingBST(root.Right)
		head := root
		p := root

		if left != nil {
			head = left
			p = head
			for p.Right != nil {
				p = p.Right
			}
			p.Right = root
			root.Right = right
		} else {
			root.Right = right
		}
		root.Left = nil
		return head
	}
	return recur(root)
}

func main() {
	fmt.Println()
}
