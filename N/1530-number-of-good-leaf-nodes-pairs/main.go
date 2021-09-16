package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func countPairs(root *TreeNode, distance int) int {

	ret := 0
	var recur func(root *TreeNode) map[int]int
	recur = func(root *TreeNode) map[int]int {
		if root.Left == nil && root.Right == nil {
			return map[int]int{1: 1}
		}

		left, right := map[int]int{0: 0}, map[int]int{0: 0}
		if root.Left != nil {
			left = recur(root.Left)
		}
		if root.Right != nil {
			right = recur(root.Right)
		}

		for k, v := range left {
			if k == 0 {
				continue
			}
			for k1, v1 := range right {
				if k1 == 0 {
					continue
				}
				if k+k1 <= distance {
					ret += v * v1
				}
			}
		}

		rm := map[int]int{}
		for k1, v1 := range left {
			if k1 == 0 {
				continue
			}
			rm[k1+1] = v1
		}
		for k1, v1 := range right {
			if k1 == 0 {
				continue
			}
			rm[k1+1] += v1
		}
		return rm
	}
	recur(root)
	return ret
}

func main() {
	fmt.Println()
}
