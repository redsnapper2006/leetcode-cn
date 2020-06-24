package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderSuccessor(root *TreeNode, p *TreeNode) *TreeNode {
	var recur func(root *TreeNode, v int, parent *TreeNode) *TreeNode
	recur = func(root *TreeNode, v int, parent *TreeNode) *TreeNode {
		if root.Val == v {
			if root.Right != nil {
				p := root.Right
				for p.Left != nil {
					p = p.Left
				}
				return p
			}
			return parent
		}
		if root.Val < v {
			return recur(root.Right, v, parent)
		}
		if root.Val > v {
			return recur(root.Left, v, root)
		}
		return nil
	}
	return recur(root, p.Val, nil)
}

func main() {
	fmt.Println("a")
}
