package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rightSideView(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	ret := []int{}
	buf := []*TreeNode{root}
	for len(buf) > 0 {
		ret = append(ret, buf[len(buf)-1].Val)
		t := []*TreeNode{}
		for _, v := range buf {
			if v.Left != nil {
				t = append(t, v.Left)
			}
			if v.Right != nil {
				t = append(t, v.Right)
			}
		}
		buf = t
	}
	return ret
}

func main() {

	fmt.Println()
}
