package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderTraversal(root *TreeNode) []int {
	if root == nil {
		return nil
	}

	var ret []int
	buf := []*TreeNode{root}
	p := root
	for p.Left != nil {
		buf = append(buf, p.Left)
		p = p.Left
	}
	for len(buf) > 0 {
		p := buf[len(buf)-1]
		buf = buf[0 : len(buf)-1]
		ret = append(ret, p.Val)
		if p.Right != nil {
			buf = append(buf, p.Right)
			q := p.Right
			for q.Left != nil {
				buf = append(buf, q.Left)
				q = q.Left
			}
		}

	}
	return ret
}

func main() {
	fmt.Println("a")
}
