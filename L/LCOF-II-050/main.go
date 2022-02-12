package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pathSum(root *TreeNode, targetSum int) int {
	if root == nil {
		return 0
	}

	ret := 0
	var recur func(root *TreeNode, sumMap map[int]int, sum int)
	recur = func(root *TreeNode, sumMap map[int]int, sum int) {
		newsum := sum + root.Val
		diff := newsum - targetSum
		ret += sumMap[diff]
		sumMap[newsum]++
		if root.Left != nil {
			recur(root.Left, sumMap, newsum)

		}
		if root.Right != nil {
			recur(root.Right, sumMap, newsum)
		}
		sumMap[newsum]--
	}
	m := map[int]int{}
	m[0] = 1
	recur(root, m, 0)
	return ret
}
