package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findLeaves(root *TreeNode) [][]int {
	ret := [][]int{}

	var recur func(root *TreeNode) int
	recur = func(root *TreeNode) int {
		if root.Left == nil && root.Right == nil {
			if len(ret) == 0 {
				ret = append(ret, []int{})
			}
			ret[0] = append(ret[0], root.Val)
			return 1
		}

		left, right := 0, 0
		if root.Left != nil {
			left = recur(root.Left)
		}
		if root.Right != nil {
			right = recur(root.Right)
		}
		var max int
		if left > 0 {
			max = left
		}
		if right > 0 && right > max {
			max = right
		}
		if len(ret) <= max {
			ret = append(ret, []int{})
		}
		ret[max] = append(ret[max], root.Val)
		return max + 1
	}

	recur(root)
	return ret
}
