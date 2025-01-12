package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSubStructure(A *TreeNode, B *TreeNode) bool {
	if B == nil {
		return false
	}
	var complete func(A *TreeNode, B *TreeNode) bool
	complete = func(A *TreeNode, B *TreeNode) bool {
		if B == nil {
			return true
		}
		if A == nil && B != nil {
			return false
		}
		if A.Val != B.Val {
			return false
		}
		return complete(A.Left, B.Left) && complete(A.Right, B.Right)
	}

	var recur func(A *TreeNode, B *TreeNode) bool
	recur = func(A *TreeNode, B *TreeNode) bool {
		if A == nil && B == nil {
			return true
		}
		if (A != nil && B == nil) || (A == nil && B != nil) {
			return false
		}

		if A.Val == B.Val {
			left, right := true, true
			if B.Left != nil {
				left = complete(A.Left, B.Left)
			}
			if B.Right != nil {
				right = complete(A.Right, B.Right)
			}
			if left && right {
				return true
			}
		}
		return recur(A.Left, B) || recur(A.Right, B)
	}
	return recur(A, B)
}

func main() {
	fmt.Println("a")
}
