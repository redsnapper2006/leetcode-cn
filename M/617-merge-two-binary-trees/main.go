package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func mergeTrees(t1 *TreeNode, t2 *TreeNode) *TreeNode {
	if t1 == nil && t2 == nil {
		return nil
	}
	v1, v2 := 0, 0
	var l1, l2 *TreeNode = nil, nil
	var r1, r2 *TreeNode = nil, nil
	if t1 != nil {
		v1 = t1.Val
		l1 = t1.Left
		r1 = t1.Right
	}
	if t2 != nil {
		v2 = t2.Val
		l2 = t2.Left
		r2 = t2.Right
	}

	tn := TreeNode{Val: v1 + v2, Left: mergeTrees(l1, l2), Right: mergeTrees(r1, r2)}
	return &tn
}

func main() {
	fmt.Println("a")
}
