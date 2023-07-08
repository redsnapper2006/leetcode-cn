package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func largestValues(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	ret := []int{}
	candi := []*TreeNode{root}
	for len(candi) > 0 {
		t := []*TreeNode{}
		max := -1 << 31
		for _, v := range candi {
			// fmt.Println(v.Val)
			if v.Val > max {
				max = v.Val
			}
			if v.Left != nil {
				t = append(t, v.Left)
			}
			if v.Right != nil {
				t = append(t, v.Right)
			}
		}
		ret = append(ret, max)
		candi = t
	}
	return ret
}
