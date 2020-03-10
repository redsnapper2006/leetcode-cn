package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func diameterOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}

	var recur func(r *TreeNode) (int, int)
	recur = func(r *TreeNode) (int, int) {
		if r == nil {
			return 0, 0
		}
		lD, lR := recur(r.Left)
		rD, rR := recur(r.Right)
		if r.Left != nil {
			lD++
		}
		if r.Right != nil {
			rD++
		}
		depth := lD
		if lD < rD {
			depth = rD
		}
		dia := lD + rD
		if lR > dia {
			dia = lR
		}
		if rR > dia {
			dia = rR
		}
		// fmt.Println(r.Val, depth, dia, lD, rD, lR, rR)
		return depth, dia
	}
	_, r := recur(root)
	return r
}

func main() {
	o1 := TreeNode{Val: 4}
	o2 := TreeNode{Val: -7}
	o3 := TreeNode{Val: -3}
	o4 := TreeNode{Val: -9}
	o5 := TreeNode{Val: -3}
	o6 := TreeNode{Val: 9}
	o7 := TreeNode{Val: -7}
	o8 := TreeNode{Val: -4}
	o9 := TreeNode{Val: 6}
	o10 := TreeNode{Val: -6}
	o11 := TreeNode{Val: -6}
	o12 := TreeNode{Val: 0}
	o13 := TreeNode{Val: 6}
	o14 := TreeNode{Val: 5}
	o15 := TreeNode{Val: 9}
	o16 := TreeNode{Val: -1}
	o17 := TreeNode{Val: -4}
	o18 := TreeNode{Val: -2}
	o1.Left = &o2
	o1.Right = &o3
	o3.Left = &o4
	o3.Right = &o5
	o4.Left = &o6
	o4.Right = &o7
	o5.Left = &o8
	o6.Left = &o9
	o7.Left = &o10
	o7.Right = &o11
	o9.Left = &o12
	o9.Right = &o13
	o10.Left = &o14
	o11.Left = &o15
	o12.Right = &o16
	o13.Left = &o17
	o15.Right = &o18

	fmt.Println(diameterOfBinaryTree(&o1))
}
