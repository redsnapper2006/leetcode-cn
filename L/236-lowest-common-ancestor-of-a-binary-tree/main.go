package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func recursiveTree(tn, p, q *TreeNode) (bool, bool, *TreeNode) /* isPFound, isQFound, rootNode */ {

	isLPFound, isLQFound := false, false
	var node *TreeNode
	if tn.Left != nil {
		isLPFound, isLQFound, node = recursiveTree(tn.Left, p, q)
	}
	if isLPFound && isLQFound {
		return isLPFound, isLQFound, node
	}

	isRPFound, isRQFound := false, false
	if tn.Right != nil {
		isRPFound, isRQFound, node = recursiveTree(tn.Right, p, q)
	}
	if isRPFound && isRQFound {
		return isRPFound, isRQFound, node
	}

	isPFound, isQFound := isLPFound || isRPFound || tn == p, isLQFound || isRQFound || tn == q

	return isPFound, isQFound, tn
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	_, _, node := recursiveTree(root, p, q)
	return node
}

func main() {
	o0, o1, o2, o3, o4, o5, o6, o7, o8 := TreeNode{Val: 0, Left: nil, Right: nil}, TreeNode{Val: 1, Left: nil, Right: nil}, TreeNode{Val: 2, Left: nil, Right: nil},
		TreeNode{Val: 3, Left: nil, Right: nil}, TreeNode{Val: 4, Left: nil, Right: nil}, TreeNode{Val: 5, Left: nil, Right: nil}, TreeNode{Val: 6, Left: nil, Right: nil}, TreeNode{Val: 7, Left: nil, Right: nil}, TreeNode{Val: 8, Left: nil, Right: nil}

	o3.Left = &o5
	o3.Right = &o1
	o1.Left = &o0
	o1.Right = &o8

	o5.Left = &o6
	o5.Right = &o2

	o2.Left = &o7
	o2.Right = &o4

	fmt.Println(lowestCommonAncestor(&o3, &o4, &o6).Val)
}
