package main

import "fmt"

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
	fmt.Println("a")
}
