package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func btreeGameWinningMove(root *TreeNode, n int, x int) bool {
	var target *TreeNode
	candi := []*TreeNode{root}
	isFound := false
	for !isFound {
		t := []*TreeNode{}
		for _, c := range candi {
			if c == nil {
				continue
			}
			if c.Val == x {
				target = c
				isFound = true
				break
			}
			t = append(t, c.Left, c.Right)
		}
		candi = t
	}

	var count func(n *TreeNode) int
	count = func(n *TreeNode) int {
		if n == nil {
			return 0
		}
		return count(n.Left) + count(n.Right) + 1
	}

	left, right := count(target.Left), count(target.Right)
	other := n - 1 - left - right
	if left > n-left || right > n-right || other > n-other {
		return true
	}
	return false
}
