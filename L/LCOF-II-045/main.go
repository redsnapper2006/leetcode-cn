package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findBottomLeftValue(root *TreeNode) int {
	buf := []*TreeNode{root}
	for len(buf) > 0 {
		t := []*TreeNode{}
		for _, n := range buf {
			if n.Left != nil {
				t = append(t, n.Left)
			}
			if n.Right != nil {
				t = append(t, n.Right)
			}
		}
		if len(t) == 0 {
			return buf[0].Val
		}
		buf = t
	}
	return -1
}

func main() {
	fmt.Println()
}
