package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrder(root *TreeNode) []int {
	if root == nil {
		return nil
	}

	var r []int
	b := []*TreeNode{root}

	for len(b) > 0 {
		var t []*TreeNode
		for i := 0; i < len(b); i++ {
			r = append(r, b[i].Val)
			if b[i].Left != nil {
				t = append(t, b[i].Left)
			}
			if b[i].Right != nil {
				t = append(t, b[i].Right)
			}
		}
		b = t
	}
	return r
}

func main() {
	fmt.Println("a")
}
