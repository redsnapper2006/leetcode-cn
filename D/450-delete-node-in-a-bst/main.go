package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func recur(root *TreeNode, parent *TreeNode, key int) {
	if root == nil {
		return
	}
	var p, pp *TreeNode
	if root.Val == key {
		if root.Right != nil {
			p = root.Right
			pp = root.Right
			for p.Left != nil {
				pp = p
				p = p.Left
			}
			p.Left = root.Left
			if pp != p {
				pp.Left = p.Right
				p.Right = root.Right
			}
		} else if root.Left != nil {
			p = root.Left
			pp = root.Left
			for p.Right != nil {
				pp = p
				p = p.Right
			}
			p.Right = root.Right
			if pp != p {
				pp.Right = p.Left
				p.Left = root.Left
			}
		} else {
			p = nil

		}

		if parent.Val > key {
			parent.Left = p
		} else {
			parent.Right = p
		}
	} else {
		if root.Val > key {
			recur(root.Left, root, key)
		} else {
			recur(root.Right, root, key)
		}
	}
}

func deleteNode(root *TreeNode, key int) *TreeNode {
	maxTemp := TreeNode{Val: 1<<63 - 1, Left: root, Right: nil}
	recur(root, &maxTemp, key)
	return maxTemp.Left
}

func main() {
	fmt.Println("a")
}
