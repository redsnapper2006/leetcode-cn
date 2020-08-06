package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumNumbers(root *TreeNode) int {
	if root == nil {
		return 0
	}

	sum := 0
	var recur func(root *TreeNode, accum int)
	recur = func(root *TreeNode, accum int) {
		if root.Left == nil && root.Right == nil {
			sum += accum*10 + root.Val
			return
		}

		if root.Left != nil {
			recur(root.Left, accum*10+root.Val)
		}
		if root.Right != nil {
			recur(root.Right, accum*10+root.Val)
		}
	}

	recur(root, 0)
	return sum
}

func main() {
	fmt.Println("a")
}
