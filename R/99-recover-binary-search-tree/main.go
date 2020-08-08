package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func recoverTree(root *TreeNode) {
	var inorder func(root *TreeNode) []*TreeNode
	inorder = func(root *TreeNode) []*TreeNode {
		if root == nil {
			return nil
		}

		left := inorder(root.Left)
		right := inorder(root.Right)
		ret := make([]*TreeNode, len(left)+len(right)+1)
		copy(ret, left)
		ret[len(left)] = root
		copy(ret[len(left)+1:], right)
		return ret
	}

	l := inorder(root)
	isFirst := true
	var p1, p2 *TreeNode = nil, nil
	for i := 0; i < len(l)-1; i++ {
		if l[i].Val > l[i+1].Val {
			if isFirst {
				p1 = l[i]
				p2 = l[i+1]
				isFirst = false
			} else {
				p2 = l[i+1]
			}
		}
	}

	p1.Val, p2.Val = p2.Val, p1.Val
}

func main() {
	fmt.Println("a")
}
