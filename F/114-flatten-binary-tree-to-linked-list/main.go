package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func flatten(root *TreeNode) {

	var recur func(root *TreeNode) *TreeNode
	recur = func(root *TreeNode) *TreeNode {
		if root == nil {
			return nil
		}
		var left, right *TreeNode = nil, nil
		if root.Left != nil {
			left = recur(root.Left)
		}
		if root.Right != nil {
			right = recur(root.Right)
		}
		HEAD := TreeNode{Val: 0, Right: root}
		root.Left = nil
		root.Right = left
		p := &HEAD
		for p.Right != nil {
			p = p.Right
		}
		p.Right = right
		return HEAD.Right
	}

	recur(root)
}

func main() {
	fmt.Println("a")
}
