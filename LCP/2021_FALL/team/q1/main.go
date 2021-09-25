package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func numColor(root *TreeNode) int {
	m := map[int]int{}

	var recur func(root *TreeNode)
	recur = func(root *TreeNode) {
		if root == nil {
			return
		}
		m[root.Val]++
		recur(root.Left)
		recur(root.Right)
	}
	recur(root)
	return len(m)
}

func main() {
	fmt.Println()
}
