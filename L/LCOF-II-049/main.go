package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumNumbers(root *TreeNode) int {
	total := 0
	var recur func(root *TreeNode, sum int)
	recur = func(root *TreeNode, sum int) {
		nSum := sum*10 + root.Val
		if root.Left == nil && root.Right == nil {
			total += nSum
			return
		}

		if root.Left != nil {
			recur(root.Left, nSum)
		}
		if root.Right != nil {
			recur(root.Right, nSum)
		}
	}

	recur(root, 0)

	return total
}

func main() {
	fmt.Println()
}
