package main

import (
	"fmt"
	"strconv"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func tree2str(t *TreeNode) string {
	if t == nil {
		return ""
	}
	if t.Left == nil && t.Right == nil {
		return strconv.FormatInt(int64(t.Val), 10)
	}

	left := ""
	if t.Left != nil {
		left = tree2str(t.Left)
	}
	right := ""
	if t.Right != nil {
		right = tree2str(t.Right)
	}

	r := strconv.FormatInt(int64(t.Val), 10)
	if left == "" {
		r += "()"
	} else {
		r += "(" + left + ")"
	}
	if right != "" {
		r += "(" + right + ")"
	}
	return r
}

func main() {
	fmt.Println("a")
}
