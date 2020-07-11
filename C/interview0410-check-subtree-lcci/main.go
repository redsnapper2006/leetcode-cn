package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func checkSubTree(t1 *TreeNode, t2 *TreeNode) bool {
	var isMatch func(t1, t2 *TreeNode) bool
	isMatch = func(t1, t2 *TreeNode) bool {
		if t1 == nil && t2 == nil {
			return true
		}
		if (t1 == nil && t2 != nil) || (t1 != nil && t2 == nil) {
			return false
		}
		if t1.Val != t2.Val {
			return false
		}
		return true
	}

	var recur func(t1, t2 *TreeNode) bool
	recur = func(t1, t2 *TreeNode) bool {
		if isMatch(t1, t2) {
			if t1 == nil && t2 == nil {
				return true
			} else if ((t1.Left == nil && t2.Left == nil) || (t1.Left != nil && t2.Left != nil)) && recur(t1.Left, t2.Left) && ((t1.Right == nil && t2.Right == nil) || (t1.Right != nil && t2.Right != nil)) && recur(t1.Right, t2.Right) {
				return true
			}
		}
		if t1 == nil {
			return false
		}
		return recur(t1.Left, t2) || recur(t1.Right, t2)
	}

	return recur(t1, t2)
}

func main() {
	fmt.Println("a")
}
