package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func generateTrees(n int) []*TreeNode {
	if n == 0 {
		return []*TreeNode{}
	}
	nums := make([]int, n)
	for i := 0; i < n; i++ {
		nums[i] = i + 1
	}

	var recur func(nums []int) []*TreeNode
	recur = func(nums []int) []*TreeNode {
		if len(nums) == 0 {
			return []*TreeNode{nil}
		}
		var buf []*TreeNode
		for i := 0; i < len(nums); i++ {
			val := nums[i]
			leftTree := recur(nums[0:i])
			rightTree := recur(nums[i+1:])

			for l := 0; l < len(leftTree); l++ {
				for r := 0; r < len(rightTree); r++ {
					buf = append(buf, &TreeNode{Val: val, Left: leftTree[l], Right: rightTree[r]})
				}
			}
		}
		return buf
	}

	return recur(nums)
}

func main() {

	fmt.Println("a")
}
