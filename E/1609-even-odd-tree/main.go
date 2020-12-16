package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isEvenOddTree(root *TreeNode) bool {
	buf := []*TreeNode{root}
	lay := 0
	for len(buf) > 0 {
		var t []int
		var tt []*TreeNode
		for _, v := range buf {
			t = append(t, v.Val)
			if v.Left != nil {
				tt = append(tt, v.Left)
			}
			if v.Right != nil {
				tt = append(tt, v.Right)
			}
		}

		s, e, ss := 0, len(t), 1
		if lay%2 == 1 {
			s, e, ss = len(t)-1, -1, -1
		}

		for i := s; i != e; i = i + ss {
			if (t[i]+1)%2 != lay%2 {
				return false
			}
		}
		for i := s + ss; i != e; i = i + ss {
			if t[i] <= t[i-ss] {
				return false
			}
		}
		buf = tt
		lay++
	}
	return true
}

func main() {
	fmt.Println()
}
