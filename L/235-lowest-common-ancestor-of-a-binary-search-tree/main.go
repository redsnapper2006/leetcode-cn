package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	var l, r *TreeNode
	if p.Val > q.Val {
		r = p
		l = q
	} else {
		l = p
		r = q
	}
	if root.Val >= l.Val && root.Val <= r.Val {
		return root
	}
	if root.Val < l.Val {
		return lowestCommonAncestor(root.Right, p, q)
	}
	return lowestCommonAncestor(root.Left, p, q)
}

func main() {
	fmt.Println("a")
}
