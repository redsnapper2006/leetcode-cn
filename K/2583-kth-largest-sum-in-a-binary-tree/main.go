package main

import "sort"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type IntArrSlice []int64

func (p IntArrSlice) Len() int {
	return len(p)
}

func (p IntArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p IntArrSlice) Less(i, j int) bool {
	return p[i] > p[j]
}

func kthLargestLevelSum(root *TreeNode, k int) int64 {
	m := map[int]int64{}

	var recur func(tn *TreeNode, l int)
	recur = func(tn *TreeNode, l int) {
		if tn == nil {
			return
		}

		m[l] += int64(tn.Val)
		recur(tn.Left, l+1)
		recur(tn.Right, l+1)
	}
	recur(root, 1)
	b := []int64{}
	for _, v := range m {
		b = append(b, v)
	}
	sort.Sort(IntArrSlice(b))

	if len(b) < k {
		return -1
	}
	return b[k-1]
}
