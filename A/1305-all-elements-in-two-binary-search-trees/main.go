package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getAllElements(root1 *TreeNode, root2 *TreeNode) []int {
	var recur2 func(root *TreeNode) []int
	recur2 = func(root *TreeNode) []int {
		if root == nil {
			return nil
		}
		left := recur2(root.Left)
		right := recur2(root.Right)
		buf := make([]int, len(left)+1+len(right))
		copy(buf, left)
		buf[len(left)] = root.Val
		copy(buf[len(left)+1:], right)
		return buf

	}
	r1, r2 := recur2(root1), recur2(root2)
	s1, s2 := 0, 0
	var ret []int
	for s1 < len(r1) && s2 < len(r2) {
		if r1[s1] < r2[s2] {
			ret = append(ret, r1[s1])
			s1++
		} else {
			ret = append(ret, r2[s2])
			s2++
		}
	}
	if s1 < len(r1) {
		for s1 < len(r1) {
			ret = append(ret, r1[s1])
			s1++
		}
	}
	if s2 < len(r2) {
		for s2 < len(r2) {
			ret = append(ret, r2[s2])
			s2++
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
