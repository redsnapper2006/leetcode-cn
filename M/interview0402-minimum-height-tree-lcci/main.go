package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sortedArrayToBST(nums []int) *TreeNode {
	size := len(nums)
	if size == 0 {
		return nil
	}
	if size == 1 {
		return &TreeNode{Val: nums[0], Left: nil, Right: nil}
	}
	if size == 2 {
		return &TreeNode{Val: nums[1], Left: &TreeNode{Val: nums[0], Left: nil, Right: nil}, Right: nil}
	}

	half := size / 2
	return &TreeNode{Val: nums[half], Left: sortedArrayToBST(nums[0:half]), Right: sortedArrayToBST(nums[half+1:])}
}

func main() {
	fmt.Println("a")
}
