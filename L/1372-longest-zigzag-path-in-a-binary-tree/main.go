package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func longestZigZag(root *TreeNode) int {
	max := 0

	var recur func(root *TreeNode) (int, int)
	recur = func(root *TreeNode) (int, int) {
		if root.Left == nil && root.Right == nil {
			return 0, 0
		}

		l, r := 0, 0
		if root.Left != nil {
			_, cr := recur(root.Left)
			l = cr + 1
		}
		if root.Right != nil {
			cl, _ := recur(root.Right)
			r = cl + 1
		}
		if l > max {
			max = l
		}
		if r > max {
			max = r
		}
		return l, r
	}
	recur(root)
	return max
}
