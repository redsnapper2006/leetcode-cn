package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumOfLeftLeaves(root *TreeNode) int {
	var recur func(root *TreeNode, sum *int)
	recur = func(root *TreeNode, sum *int) {
		if root == nil {
			return
		}
		if root.Left != nil && root.Left.Left == nil && root.Left.Right == nil {
			*sum += root.Left.Val
		}
		recur(root.Left, sum)
		recur(root.Right, sum)
	}
	sum := 0
	recur(root, &sum)
	return sum
}

func main() {
	fmt.Println("a")
}
