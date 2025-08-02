package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getNumber(root *TreeNode, ops [][]int) int {
	i := len(ops) - 1

	var merge func(root *TreeNode) []int
	merge = func(root *TreeNode) []int {
		if root == nil {
			return nil
		}
		left := merge(root.Left)
		right := merge(root.Right)
		return append(append(left, root.Val), right...)
	}
	coll := merge(root)

	bs := func(coll []int, t int) int {
		s := 0
		e := len(coll) - 1
		for s <= e {
			m := s + (e-s)/2
			if coll[m] >= t {
				e = m - 1
			} else {
				s = m + 1
			}
		}
		return s
	}

	cnt := 0
	for i >= 0 {
		t := ops[i][0]
		s := ops[i][1]
		e := ops[i][2]

		l := bs(coll, s)
		r := bs(coll, e)
		if r == len(coll) || coll[r] > e {
			r--
		}
		if t == 1 {
			cnt += r - l + 1
		}
		coll = append(coll[0:l], coll[r+1:]...)
		i--
	}

	return cnt
}
