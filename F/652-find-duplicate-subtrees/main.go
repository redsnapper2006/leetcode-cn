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

func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	var recur func(root *TreeNode, M map[string]int, MT map[string]*TreeNode) string
	recur = func(root *TreeNode, M map[string]int, MT map[string]*TreeNode) string {
		if root == nil {
			return "nil"
		}

		l := recur(root.Left, M, MT)
		r := recur(root.Right, M, MT)
		k := strconv.FormatInt(int64(root.Val), 10) + "#" + l + "#" + r
		M[k]++
		MT[k] = root
		return k
	}

	M := make(map[string]int)
	MT := make(map[string]*TreeNode)
	recur(root, M, MT)
	var b []*TreeNode
	for k, v := range M {
		if v > 1 {
			b = append(b, MT[k])
		}
	}

	return b
}

func main() {
	fmt.Println("a")
}
