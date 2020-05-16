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

func printTree(root *TreeNode) [][]string {
	var depth func(root *TreeNode) int
	depth = func(root *TreeNode) int {
		if root == nil {
			return 0
		}

		left := depth(root.Left)
		right := depth(root.Right)
		if left > right {
			return left + 1
		}
		return right + 1
	}
	Depth := depth(root)
	Col := 1
	for i := 0; i < Depth; i++ {
		Col *= 2
	}
	Col--
	var ret [][]string
	stack := []*TreeNode{root}
	half := 1
	for len(stack) > 0 {
		half *= 2
		start := Col / half
		steps := (Col + 1) / half * 2
		t := make([]string, Col)
		for i := 0; i < Col; i++ {
			t[i] = ""
		}
		for idx, i := 0, start; i < Col; i, idx = i+steps, idx+1 {
			if stack[idx] != nil {
				t[i] = strconv.FormatInt(int64(stack[idx].Val), 10)
			}
		}
		ret = append(ret, t)
		isAllNil := true
		st := make([]*TreeNode, len(stack)*2)
		for i := 0; i < len(stack); i++ {
			if stack[i] == nil {
				st[i*2] = nil
				st[i*2+1] = nil
			} else {
				if stack[i].Left != nil || stack[i].Right != nil {
					isAllNil = false
				}
				st[i*2] = stack[i].Left
				st[i*2+1] = stack[i].Right
			}
		}
		stack = st
		if isAllNil {
			break
		}
	}

	return ret
}

func main() {
	fmt.Println("a")
}
