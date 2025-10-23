package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minimumOperations(root *TreeNode) int {
	ans := 0

	tn := []*TreeNode{root}
	for len(tn) > 0 {
		v := []int{}
		t := []*TreeNode{}
		for _, n := range tn {
			v = append(v, n.Val)
			if n.Left != nil {
				t = append(t, n.Left)
			}
			if n.Right != nil {
				t = append(t, n.Right)
			}
		}

		for i := 0; i < len(v); i++ {
			mn := v[i]
			idx := i
			for j := i + 1; j < len(v); j++ {
				if mn > v[j] {
					mn = v[j]
					idx = j
				}
			}
			if idx != i {
				v[i], v[idx] = v[idx], v[i]
				ans++
			}
		}

		tn = t
	}
	return ans
}
