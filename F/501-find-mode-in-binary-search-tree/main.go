package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findMode(root *TreeNode) []int {

	m := map[int]int{}

	var recur func(root *TreeNode)
	recur = func(root *TreeNode) {
		if root == nil {
			return
		}
		m[root.Val]++

		recur(root.Left)
		recur(root.Right)
	}
	recur(root)
	max := 0
	for _, v := range m {
		if v > max {
			max = v
		}
	}

	var ret []int
	for k, v := range m {
		if v == max {
			ret = append(ret, k)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
