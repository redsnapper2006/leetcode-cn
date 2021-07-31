package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxLevelSum(root *TreeNode) int {
	level := 1
	max := root.Val

	buf := []*TreeNode{root}
	curLevel := 0
	for len(buf) > 0 {
		curLevel++
		t := []*TreeNode{}
		sum := 0
		for _, c := range buf {
			sum += c.Val
			if c.Left != nil {
				t = append(t, c.Left)
			}
			if c.Right != nil {
				t = append(t, c.Right)
			}
		}
		if sum > max {
			level = curLevel
			max = sum
		}
		buf = t
	}
	return level
}

func main() {
	fmt.Println()
}
