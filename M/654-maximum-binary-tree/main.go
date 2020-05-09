package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func constructMaximumBinaryTree(nums []int) *TreeNode {
	var recur func(nums []int) *TreeNode
	recur = func(nums []int) *TreeNode {
		if len(nums) == 0 {
			return nil
		}

		max := nums[0]
		idx := 0
		for i := 1; i < len(nums); i++ {
			if nums[i] > max {
				max = nums[i]
				idx = i
			}
		}
		return &TreeNode{Val: max, Left: recur(nums[0:idx]), Right: recur(nums[idx+1:])}
	}
	return recur(nums)
}

func main() {
	fmt.Println("a")
}
