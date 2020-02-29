package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isMirror(left, right *TreeNode) bool {
	if !((left == nil && right == nil) || (left != nil && right != nil && left.Val == right.Val)) {
		return false
	}
	if left == nil && right == nil {
		return true
	}
	return isMirror(left.Left, right.Right) && isMirror(left.Right, right.Left)
}

func isSymmetric(root *TreeNode) bool {
	if root == nil {
		return true
	}

	return isMirror(root.Left, root.Right)
}

func isSymmetricV2(root *TreeNode) bool {
	if root == nil {
		return true
	}

	var b []*TreeNode
	b = append(b, root)
	for len(b) > 0 {
		if len(b) > 1 {
			if len(b)%2 != 0 {
				return false
			}
			s, e := 0, len(b)-1
			for s < e {
				if !((b[s] == nil && b[e] == nil) || (b[s] != nil && b[e] != nil && b[s].Val == b[e].Val)) {
					return false
				}
				s++
				e--
			}
		}
		var t []*TreeNode
		for i := 0; i < len(b); i++ {
			if b[i] != nil {
				t = append(t, b[i].Left, b[i].Right)
			}
		}
		b = t
	}
	return true
}

func main() {
	fmt.Println("a")
}
