package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumEvenGrandparent(root *TreeNode) int {
	if root == nil {
		return 0
	}
	sum := 0
	var recur func(root *TreeNode, parent, grandParent *TreeNode)
	recur = func(root *TreeNode, parent, grandParent *TreeNode) {
		if root == nil {
			return
		}
		if grandParent != nil && grandParent.Val%2 == 0 {
			sum += root.Val
		}
		recur(root.Left, root, parent)
		recur(root.Right, root, parent)
	}
	recur(root, nil, nil)
	return sum
}

func main() {
	fmt.Println()
}
