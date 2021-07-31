package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func delNodes(root *TreeNode, to_delete []int) []*TreeNode {
	ret := []*TreeNode{root}

	var recur func(root *TreeNode, to_delete []int) bool
	recur = func(root *TreeNode, to_delete []int) bool {
		if root == nil {
			return false
		}
		isTarget := false
		for _, v := range to_delete {
			if root.Val == v {
				isTarget = true
				break
			}
		}
		if isTarget {
			if root.Left != nil {
				ret = append(ret, root.Left)
			}
			if root.Right != nil {
				ret = append(ret, root.Right)
			}
		}
		left := recur(root.Left, to_delete)
		if left {
			root.Left = nil
		}
		right := recur(root.Right, to_delete)
		if right {
			root.Right = nil
		}
		return isTarget
	}
	recur(root, to_delete)

	buf := []*TreeNode{}
	for _, v := range ret {
		isMatch := false
		for _, d := range to_delete {
			if v.Val == d {
				isMatch = true
				break
			}
		}
		if !isMatch {
			buf = append(buf, v)
		}
	}
	return buf
}

func main() {
	fmt.Println()
}
