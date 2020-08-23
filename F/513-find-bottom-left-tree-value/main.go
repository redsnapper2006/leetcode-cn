package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findBottomLeftValue(root *TreeNode) int {
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		var t []*TreeNode
		for i := 0; i < len(stack); i++ {
			b := stack[i]
			if b.Left != nil {
				t = append(t, b.Left)
			}
			if b.Right != nil {
				t = append(t, b.Right)
			}
		}
		if len(t) == 0 {
			return stack[0].Val
		}
		stack = t
	}
	return -1
}

func main() {
	fmt.Println("a")
}
