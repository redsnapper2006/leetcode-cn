package main

import "sort"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func kthLargestPerfectSubtree(root *TreeNode, k int) int {

	valid := func(n int) bool {
		off := 31
		for off >= 0 {
			if n == (1 << off) {
				return true
			}
			off -= 1
		}
		return false
	}

	var recur func(*TreeNode, *[]int) (int, int)
	recur = func(root *TreeNode, buf *[]int) (int, int) {
		if root == nil {
			return 0, 0
		}

		left, ll := recur(root.Left, buf)
		right, rl := recur(root.Right, buf)
		v := left + right + 1
		if left == right && ll == rl && valid(v+1) {
			*buf = append(*buf, v)
		}
		return v, max(ll, rl) + 1
	}

	buf := []int{}
	recur(root, &buf)
	sort.Ints(buf)
	if len(buf) < k {
		return -1
	}
	return buf[len(buf)-k]
}
