package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rightSideView(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	var r []int
	p := []*TreeNode{root}

	for len(p) > 0 {
		r = append(r, p[len(p)-1].Val)
		var t []*TreeNode

		for i := 0; i < len(p); i++ {
			if p[i].Left != nil {
				t = append(t, p[i].Left)
			}
			if p[i].Right != nil {
				t = append(t, p[i].Right)
			}
		}
		p = t
	}
	return r
}

func main() {
	fmt.Println("a")
}
