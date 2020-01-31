package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func recursiveTree(tn *TreeNode, k int, buf *([]int)) int {
	if len(*buf) >= k {
		return (*buf)[k-1]
	}

	if tn.Left != nil {
		_ = recursiveTree(tn.Left, k, buf)
	}
	if len(*buf) >= k {
		return (*buf)[k-1]
	}

	*buf = append(*buf, tn.Val)
	if len(*buf) >= k {
		return (*buf)[k-1]
	}

	if tn.Right != nil {
		_ = recursiveTree(tn.Right, k, buf)
	}
	if len(*buf) >= k {
		return (*buf)[k-1]
	}
	return 0
}

func kthSmallest(root *TreeNode, k int) int {
	return recursiveTree(root, k, &[]int{})
}

func main() {
	o1, o2, o3, o4, o5, o6 := TreeNode{Val: 1, Left: nil, Right: nil}, TreeNode{Val: 2, Left: nil, Right: nil},
		TreeNode{Val: 3, Left: nil, Right: nil}, TreeNode{Val: 4, Left: nil, Right: nil}, TreeNode{Val: 5, Left: nil, Right: nil}, TreeNode{Val: 6, Left: nil, Right: nil}

	o5.Left = &o3
	o5.Right = &o6

	o3.Left = &o2
	o3.Right = &o4
	o2.Left = &o1

	fmt.Println(kthSmallest(&o5, 3))
}
