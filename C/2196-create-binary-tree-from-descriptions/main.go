package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func createBinaryTree(descriptions [][]int) *TreeNode {
	nm := map[int]*TreeNode{}
	parents := map[int]int{}

	for _, des := range descriptions {
		p, c, l := des[0], des[1], des[2]
		if _, ok := nm[p]; !ok {
			nm[p] = &TreeNode{Val: p}
		}
		if _, ok := nm[c]; !ok {
			nm[c] = &TreeNode{Val: c}
		}
		if l == 1 {
			nm[p].Left = nm[c]
		} else {
			nm[p].Right = nm[c]
		}
		parents[c] = p
	}

	start := descriptions[0][0]
	_, ok := parents[start]

	for ok {
		start = parents[start]
		_, ok = parents[start]
	}

	return nm[start]

}
