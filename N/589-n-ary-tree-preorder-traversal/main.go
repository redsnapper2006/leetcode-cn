package main

import (
	"fmt"
)

type Node struct {
	Val      int
	Children []*Node
}

func preorder(root *Node) []int {

	if root == nil {
		return []int{}
	}

	stack := []*Node{root}
	result := []int{}
	for len(stack) > 0 {
		p := stack[len(stack)-1]
		stack = stack[0 : len(stack)-1]
		result = append(result, p.Val)
		for i := len(p.Children) - 1; i >= 0; i-- {
			stack = append(stack, p.Children[i])
		}
	}
	return result
}
func main() {
	fmt.Println("a")
}
