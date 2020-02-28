package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func postorderTraversal(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	stack := []*TreeNode{root}
	buf := []int{}

	for len(stack) > 0 {
		p := stack[len(stack)-1]
		stack = stack[0 : len(stack)-1]
		buf = append(buf, p.Val)
		if p.Left != nil {
			stack = append(stack, p.Left)
		}
		if p.Right != nil {
			stack = append(stack, p.Right)
		}
	}

	s, e := 0, len(buf)-1
	for s < e {
		buf[s], buf[e] = buf[e], buf[s]
		s++
		e--
	}
	return buf
}

func main() {
	fmt.Println("a")
}
