package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func countNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}
	var recur func(root *TreeNode, c *int)
	recur = func(root *TreeNode, c *int) {
		if root == nil {
			return
		}
		(*c)++
		recur(root.Left, c)
		recur(root.Right, c)
	}
	total := 0
	recur(root, &total)
	return total
}

func main() {
	fmt.Println("a")
}
