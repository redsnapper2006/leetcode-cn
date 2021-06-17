package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func addOneRow(root *TreeNode, v int, d int) *TreeNode {
	if d == 1 {
		return &TreeNode{Val: v, Left: root}
	}

	NH := TreeNode{Val: v, Left: root}
	p := &NH
	stack := []*TreeNode{p}
	for d > 1 {
		var t []*TreeNode
		for i := 0; i < len(stack); i++ {
			if stack[i].Left != nil {
				t = append(t, stack[i].Left)
			}
			if stack[i].Right != nil {
				t = append(t, stack[i].Right)
			}
		}
		stack = t
		d--
	}
	for i := 0; i < len(stack); i++ {
		l := stack[i].Left
		stack[i].Left = &TreeNode{Val: v, Left: l}
		r := stack[i].Right
		stack[i].Right = &TreeNode{Val: v, Right: r}
	}
	return NH.Left
}

func main() {
	fmt.Println("a")
}
