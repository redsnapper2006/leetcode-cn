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
		if root.Val != head.Val {
			return false
		}

		return recur(head.Next, root.Left) || recur(head.Next, root.Right)
	}

	var r2 func(root *TreeNode) int
	r2 = func(root *TreeNode) int {
		if root == nil {
			return 0
		}
		l := r2(root.Left)
		r := r2(root.Right)
		ret := l
		if ret < r {
			ret = r
		}
		return ret + 1
	}
	var h2 func(head *ListNode) int
	h2 = func(head *ListNode) int {
		h := head
		r := 0
		for h != nil {
			r++
			h = h.Next
		}
		return r
	}

	if h2(head) > r2(root) {
		return false
	}
	return recur(head, root) || isSubPath(head, root.Left) || isSubPath(head, root.Right)
}

func main() {
	fmt.Println()
}
