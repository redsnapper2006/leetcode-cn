package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func averageOfLevels(root *TreeNode) []float64 {
	if root == nil {
		return nil
	}

	var ret []float64
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		sum := 0
		var t []*TreeNode
		for i := 0; i < len(stack); i++ {
			sum += stack[i].Val
			if stack[i].Left != nil {
				t = append(t, stack[i].Left)
			}
			if stack[i].Right != nil {
				t = append(t, stack[i].Right)
			}
		}

		ret = append(ret, float64(sum)/float64(len(stack)))
		stack = t
	}
	return ret
}

func main() {
	fmt.Println("a")
}
