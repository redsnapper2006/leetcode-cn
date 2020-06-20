package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isBalanced(root *TreeNode) bool {
	var count func(r *TreeNode) (int, bool)
	count = func(r *TreeNode) (int, bool) {
		if r == nil {
			return 0, true
		}

		left, right := 0, 0
		lBal, rBal := true, true
		if r.Left != nil {
			left, lBal = count(r.Left)
		}
		if r.Right != nil {
			right, rBal = count(r.Right)
		}
		if !lBal || !rBal {
			return 0, false
		}

		if left-right > 1 || right-left > 1 {
			return 0, false
		}
		if left > right {
			return left + 1, true
		}
		return right + 1, true
	}
	_, r := count(root)
	return r
}

func main() {
	fmt.Println("a")
}
