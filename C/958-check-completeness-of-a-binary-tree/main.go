package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isCompleteTree(root *TreeNode) bool {
	if root == nil {
		return true
	}
	buf := []*TreeNode{root}

	level := 1

	for len(buf) > 0 {
		var t []*TreeNode
		for i := 0; i < len(buf); i++ {
			t = append(t, buf[i].Left, buf[i].Right)
		}
		if len(t) != 1<<level {
			return false
		}
		isFirstNil := false
		var tt []*TreeNode
		for i := 0; i < len(t); i++ {
			if t[i] == nil {
				if !isFirstNil {
					isFirstNil = true
				}
			} else {
				if isFirstNil {
					return false
				}
				tt = append(tt, t[i])
			}
		}
		buf = tt
		if len(buf) != 1<<level {
			break
		}
		level++
	}
	for i := 0; i < len(buf); i++ {
		if buf[i].Left != nil || buf[i].Right != nil {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("hello world")
}
