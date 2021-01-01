package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func goodNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}
	var recur func(root *TreeNode, max int, ret *int)
	recur = func(root *TreeNode, max int, ret *int) {
		if root == nil {
			return
		}
		if root.Val >= max {
			*ret++
			max = root.Val
		}
		recur(root.Left, max, ret)
		recur(root.Right, max, ret)
	}
	ret := 0
	recur(root, root.Val, &ret)
	return ret
}

func main() {
	fmt.Println()
}
