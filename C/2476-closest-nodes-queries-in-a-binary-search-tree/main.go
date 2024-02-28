package main

import "sort"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func closestNodes(root *TreeNode, queries []int) [][]int {
	var recur func(tn *TreeNode, buf *[]int)
	recur = func(tn *TreeNode, buf *[]int) {
		if tn == nil {
			return
		}

		recur(tn.Left, buf)
		recur(tn.Right, buf)
		*buf = append(*buf, tn.Val)
	}

	buf := []int{}
	recur(root, &buf)
	sort.Ints(buf)
	ret := [][]int{}
	for _, q := range queries {
		s, e := 0, len(buf)-1

		for s <= e {
			m := s + (e-s)/2
			if buf[m] >= q {
				e = m - 1
			} else {
				s = m + 1
			}
		}

		if s == len(buf) {
			ret = append(ret, []int{buf[s-1], -1})
		} else {
			if buf[s] == q {
				ret = append(ret, []int{buf[s], buf[s]})
			} else if s == 0 {
				ret = append(ret, []int{-1, buf[s]})
			} else {
				ret = append(ret, []int{buf[s-1], buf[s]})
			}
		}

	}
	return ret
}
