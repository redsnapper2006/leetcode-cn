package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func insertIntoBST(root *TreeNode, val int) *TreeNode {
	if root == nil || root.Val == val {
		root.Val = val
		return root
	}

	p := root
	pp := root
	for p != nil {
		if p.Val > val {
			pp = p
			p = p.Left
		} else if p.Val < val {
			pp = p
			p = p.Right
		} else {
			return root
		}
	}
	t := TreeNode{Val: val, Left: nil, Right: nil}
	if pp.Val > val {
		pp.Left = &t
	} else {
		pp.Right = &t
	}
	return root
}

func main() {
	fmt.Println("a")
}
