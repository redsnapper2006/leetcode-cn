package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {
	if root == nil {
		return nil
	}

	var ret []int
	buf := []*TreeNode{root}
	for len(buf) > 0 {
		p := buf[len(buf)-1]
		buf = buf[0 : len(buf)-1]
		ret = append(ret, p.Val)
		if p.Right != nil {
			buf = append(buf, p.Right)
		}
		if p.Left != nil {
			buf = append(buf, p.Left)
		}
	}
	return ret
}

func main() {
	o1, o2, o3, o4 := TreeNode{Val: 1, Left: nil, Right: nil}, TreeNode{Val: 4, Left: nil, Right: nil}, TreeNode{Val: 3, Left: nil, Right: nil}, TreeNode{Val: 2, Left: nil, Right: nil}
	o1.Left = &o2
	o1.Right = &o3
	o2.Left = &o4
	fmt.Println(preorderTraversal(&o1))
}
