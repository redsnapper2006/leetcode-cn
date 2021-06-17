package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sortedListToBST(head *ListNode) *TreeNode {

	findMid := func(root *ListNode) (*ListNode, *ListNode) {
		if root == nil {
			return nil, nil
		}
		var parent *ListNode
		slow, fast := root, root
		for fast.Next != nil && fast.Next.Next != nil {
			parent = slow
			slow = slow.Next
			fast = fast.Next.Next
		}
		return slow, parent
	}

	var buildTree func(head *ListNode) *TreeNode
	buildTree = func(head *ListNode) *TreeNode {
		if head == nil {
			return nil
		}
		mid, parent := findMid(head)
		if parent == nil {
			head := TreeNode{Val: mid.Val}
			if mid.Next != nil {
				head.Right = &TreeNode{Val: mid.Next.Val}
			}
			return &head
		}

		tHead := TreeNode{Val: mid.Val}
		parent.Next = nil
		left := buildTree(head)
		right := buildTree(mid.Next)
		tHead.Left = left
		tHead.Right = right
		return &tHead
	}

	return buildTree(head)
}

func main() {
	fmt.Println("a")
}
