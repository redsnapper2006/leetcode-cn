package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findTilt(root *TreeNode) int {

	tilt := 0
	var recur func(root *TreeNode) int
	recur = func(root *TreeNode) int {
		if root == nil {
			return 0
		}

		left := recur(root.Left)
		right := recur(root.Right)
		if left > right {
			tilt += left - right
		} else {
			tilt += right - left
		}
		return root.Val + left + right
	}

	recur(root)
	return tilt
}

func main() {
	fmt.Println("a")
}
