package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func convertBST(root *TreeNode) *TreeNode {
	var recur func(root *TreeNode, sum *int)
	recur = func(root *TreeNode, sum *int) {
		if root == nil {
			return
		}
		recur(root.Right, sum)
		*sum += root.Val
		root.Val = *sum
		recur(root.Left, sum)
	}

	s := 0
	recur(root, &s)
	return root
}

func main() {
	fmt.Println("a")
}
