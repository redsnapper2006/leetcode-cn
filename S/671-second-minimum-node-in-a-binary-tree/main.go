package main

import (
	"fmt"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findSecondMinimumValue(root *TreeNode) int {
	if root == nil {
		return -1
	}
	var recur func(root *TreeNode) []int
	recur = func(root *TreeNode) []int {
		if root.Left == nil && root.Right == nil {
			return []int{root.Val}
		}

		left := recur(root.Left)
		right := recur(root.Right)
		return append(left, right...)
	}
	a := recur(root)
	sort.Ints(a)
	if len(a) <= 1 {
		return -1
	}
	min1 := a[0]
	min2 := -1
	for i := 1; i < len(a); i++ {
		if min2 == -1 && a[i] > min1 {
			min2 = a[i]
		}
	}

	return min2
}

func main() {
	fmt.Println("a")
}
