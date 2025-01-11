package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func kthLargest(root *TreeNode, k int) int {
	var count func(root *TreeNode) int
	count = func(root *TreeNode) int {
		if root == nil {
			return 0
		}
		return count(root.Left) + 1 + count(root.Right)
	}
	var recur func(root *TreeNode, k int) int
	recur = func(root *TreeNode, k int) int {
		right := count(root.Right)
		if right >= k {
			return recur(root.Right, k)
		}
		if right+1 == k {
			return root.Val
		}
		return recur(root.Left, k-right-1)
	}

	return recur(root, k)
}

func main() {
	fmt.Println("a")
}
