package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func averageOfSubtree(root *TreeNode) int {
	cnt := 0

	var recur func(root *TreeNode) (int, int)
	recur = func(root *TreeNode) (int, int) {
		if root == nil {
			return 0, 0
		}

		leftSum, leftCnt := recur(root.Left)
		rightSum, rightCnt := recur(root.Right)

		sum := leftSum + rightSum + root.Val
		count := leftCnt + rightCnt + 1
		if sum/count == root.Val {
			cnt++
		}

		return sum, count
	}
	recur(root)

	return cnt
}
