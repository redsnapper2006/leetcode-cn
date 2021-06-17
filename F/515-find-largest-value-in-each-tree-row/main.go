package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func largestValues(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	var ret []int
	buf := []*TreeNode{root}
	for len(buf) > 0 {
		max := buf[0].Val
		var t []*TreeNode
		if buf[0].Left != nil {
			t = append(t, buf[0].Left)
		}
		if buf[0].Right != nil {
			t = append(t, buf[0].Right)
		}
		for i := 1; i < len(buf); i++ {
			if max < buf[i].Val {
				max = buf[i].Val
			}
			if buf[i].Left != nil {
				t = append(t, buf[i].Left)
			}
			if buf[i].Right != nil {
				t = append(t, buf[i].Right)
			}
		}
		ret = append(ret, max)
		buf = t
	}
	return ret
}

func main() {
	fmt.Println("a")
}
