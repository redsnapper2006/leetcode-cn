package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func reverseOddLevels(root *TreeNode) *TreeNode {
	parent := []*TreeNode{root}
	for len(parent) > 0 {
		if parent[0].Left == nil {
			break
		}
		next := []*TreeNode{}
		for i := 0; i < len(parent); i++ {
			next = append(next, parent[i].Left, parent[i].Right)
		}
		t := []*TreeNode{}
		for i := 0; i < len(next)/2; i++ {
			lp, rp := next[i], next[len(next)-1-i]
			lidx, ridx := i/2, (len(next)-1-i)/2
			if i%2 == 0 {
				parent[lidx].Left = rp
				parent[ridx].Right = lp
			} else {
				parent[lidx].Right = rp
				parent[ridx].Left = lp
			}
			lpl, lpr := lp.Left, lp.Right
			rpl, rpr := rp.Left, rp.Right

			lp.Left = rpl
			lp.Right = rpr
			rp.Left = lpl
			rp.Right = lpr
		}
		for i := 0; i < len(parent); i++ {
			if parent[i].Left != nil && parent[i].Left.Left != nil {
				t = append(t, parent[i].Left.Left, parent[i].Left.Right)
			}
			if parent[i].Right != nil && parent[i].Right.Left != nil {
				t = append(t, parent[i].Right.Left, parent[i].Right.Right)
			}
		}
		parent = t
	}
	return root
}
