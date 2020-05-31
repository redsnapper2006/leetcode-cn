package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumRootToLeaf(root *TreeNode) int {
	var recur func(root *TreeNode, hier int, sum *int)
	recur = func(root *TreeNode, hier int, sum *int) {
		if root == nil {
			return
		}
		if root.Left == nil && root.Right == nil {
			*sum += hier*2 + root.Val
			return
		}
		recur(root.Left, hier*2+root.Val, sum)
		recur(root.Right, hier*2+root.Val, sum)
	}
	sum := 0
	recur(root, 0, &sum)
	return sum
}

func main() {
	fmt.Println("a")
}
