package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rob(root *TreeNode) int {
	if root == nil {
		return 0
	}
	max := root.Val
	var recur func(root *TreeNode) (int, int, int)
	recur = func(root *TreeNode) (int, int, int) {
		if root.Left == nil && root.Right == nil {
			return 0, 0, root.Val
		}

		leftPrevPrev, leftPrev, left := 0, 0, 0
		if root.Left != nil {
			leftPrevPrev, leftPrev, left = recur(root.Left)
		}

		rightPrevPrev, rightPrev, right := 0, 0, 0
		if root.Right != nil {
			rightPrevPrev, rightPrev, right = recur(root.Right)
		}

		r := leftPrevPrev + rightPrevPrev
		if r < leftPrev+rightPrev {
			r = leftPrev + rightPrev
		}
		if r < leftPrev+rightPrevPrev {
			r = leftPrev + rightPrevPrev
		}
		if r < leftPrevPrev+rightPrev {
			r = leftPrevPrev + rightPrev
		}
		p := left + right
		if p < left+rightPrev {
			p = left + rightPrev
		}
		if p < leftPrev+right {
			p = leftPrev + right
		}
		pp := leftPrev + rightPrev
		if pp < leftPrev+right {
			pp = leftPrev + right
		}
		if pp < left+rightPrev {
			pp = left + rightPrev
		}
		if max < pp {
			max = pp
		}
		if max < p {
			max = p
		}
		if max < r+root.Val {
			max = r + root.Val
		}
		return pp, p, r + root.Val
	}
	recur(root)
	return max
}

func main() {
	fmt.Println("a")
}
