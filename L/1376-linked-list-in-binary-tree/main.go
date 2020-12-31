package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSubPath(head *ListNode, root *TreeNode) bool {
	var recur func(head *ListNode, root *TreeNode) bool
	recur = func(head *ListNode, root *TreeNode) bool {
		if head == nil {
			return true
		}
		if root == nil {
			return false
		}
		if root.Val == head.Val {
			return recur(head.Next, root.Left) || recur(head.Next, root.Right)
		}
		return recur(head, root.Left) || recur(head, root.Right)
	}
	return recur(head, root)
}

func main() {
	fmt.Println()
}
