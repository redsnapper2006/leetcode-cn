package main

import (
	"fmt"
)

type Node struct {
	Val      int
	Children []*Node
}

func postorder(root *Node) []int {
	if root == nil {
		return []int{}
	}

	stack := []*Node{root}
	result := []int{}
	for len(stack) > 0 {
		p := stack[len(stack)-1]
		stack = stack[0 : len(stack)-1]
		result = append(result, p.Val)
		for i := 0; i < len(p.Children); i++ {
			stack = append(stack, p.Children[i])
		}
	}
	s, e := 0, len(result)-1
	for s < e {
		result[s], result[e] = result[e], result[s]
		s++
		e--
	}
	return result
}

func main() {
	fmt.Println("a")
}
