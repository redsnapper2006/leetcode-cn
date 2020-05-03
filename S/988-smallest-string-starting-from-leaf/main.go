package main

import (
	"fmt"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func smallestFromLeaf(root *TreeNode) string {
	if root == nil {
		return ""
	}

	var buf []string
	var recur func(r *TreeNode, str string)
	recur = func(r *TreeNode, str string) {
		if r.Left == nil && r.Right == nil {
			str = string([]byte{byte(r.Val + 'a')}) + str
			buf = append(buf, str)
		}

		if r.Left != nil {
			recur(r.Left, string([]byte{byte(r.Val + 'a')})+str)
		}
		if r.Right != nil {
			recur(r.Right, string([]byte{byte(r.Val + 'a')})+str)
		}
	}
	recur(root, "")
	sort.Strings(buf)
	return buf[0]
}

func main() {
	fmt.Println("a")
}
