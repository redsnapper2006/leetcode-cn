package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxPathSumImpl(root *TreeNode) (int, int) {
	left, right, leftMax, rightMax := 0, 0, 0, 0

	max := root.Val
	if root.Left != nil {
		left, leftMax = maxPathSumImpl(root.Left)
		if max < left+root.Val {
			max = left + root.Val
		}
	}
	if root.Right != nil {
		right, rightMax = maxPathSumImpl(root.Right)
		if max < right+root.Val {
			max = right + root.Val
		}
	}

	bufMax := max
	if root.Left != nil {
		if bufMax < leftMax {
			bufMax = leftMax
		}
	}
	if root.Right != nil {
		if bufMax < rightMax {
			bufMax = rightMax
		}
	}
	if root.Left != nil && root.Right != nil {
		if bufMax < root.Val+root.Left.Val+root.Right.Val {
			bufMax = root.Val + root.Left.Val + root.Right.Val
		}
		if bufMax < root.Val+left+right {
			bufMax = root.Val + left + right
		}
	}

	return max, bufMax
}

func maxPathSum(root *TreeNode) int {
	_, max := maxPathSumImpl(root)
	return max
}

func main() {

	// o1, o2, o3, o4, o5 := TreeNode{Val: -10, Left: nil, Right: nil}, TreeNode{Val: 9, Left: nil, Right: nil}, TreeNode{Val: 20, Left: nil, Right: nil}, TreeNode{Val: 15, Left: nil, Right: nil}, TreeNode{Val: 7, Left: nil, Right: nil}

	// o1.Left = &o2
	// o1.Right = &o3
	// o3.Left = &o4
	// o3.Right = &o5

	o1, o2, o3, o4, o5, o6, o7, o8, o9 := TreeNode{Val: 5, Left: nil, Right: nil}, TreeNode{Val: 4, Left: nil, Right: nil}, TreeNode{Val: 8, Left: nil, Right: nil}, TreeNode{Val: 11, Left: nil, Right: nil}, TreeNode{Val: 13, Left: nil, Right: nil}, TreeNode{Val: 4, Left: nil, Right: nil}, TreeNode{Val: 7, Left: nil, Right: nil}, TreeNode{Val: 2, Left: nil, Right: nil}, TreeNode{Val: 1, Left: nil, Right: nil}

	o1.Left = &o2
	o1.Right = &o3
	o2.Left = &o4
	o3.Left = &o5
	o3.Right = &o6
	o4.Left = &o7
	o4.Right = &o8
	o6.Right = &o9

	fmt.Println(maxPathSum(&o1))
}
