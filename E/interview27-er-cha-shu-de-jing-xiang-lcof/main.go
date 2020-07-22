package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func mirrorTree(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}

	root.Right, root.Left = mirrorTree(root.Left), mirrorTree(root.Right)
	return root
}

func main() {
	fmt.Println("a")
}
