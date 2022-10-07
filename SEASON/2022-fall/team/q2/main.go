package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func expandBinaryTree(root *TreeNode) *TreeNode {

	var recur func(root *TreeNode)
	recur = func(root *TreeNode) {
		if root == nil {
			return
		}
		ol, or := root.Left, root.Right
		if ol != nil {
			root.Left = &TreeNode{Val: -1, Left: ol}
			recur(ol)
		}
		if or != nil {
			root.Right = &TreeNode{Val: -1, Right: or}
			recur(or)
		}
	}

	recur(root)
	return root
}
