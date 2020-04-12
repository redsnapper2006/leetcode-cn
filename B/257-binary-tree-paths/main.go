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

func binaryTreePaths(root *TreeNode) []string {
	if root == nil {
		return []string{}
	}
	var recur func(root *TreeNode, s string, result *[]string)
	recur = func(root *TreeNode, s string, result *[]string) {
		if root.Left == nil && root.Right == nil {
			*result = append(*result, s+strconv.FormatInt(int64(root.Val), 10))
			return
		}
		s += strconv.FormatInt(int64(root.Val), 10) + "->"
		if root.Left != nil {
			recur(root.Left, s, result)
		}
		if root.Right != nil {
			recur(root.Right, s, result)
		}
	}
	var r []string
	recur(root, "", &r)
	return r
}

func main() {
	fmt.Println("a")
}
