package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrderBottom(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}

	var r [][]int
	var b []*TreeNode

	b = append(b, root)
	for len(b) > 0 {
		var l []int
		var t []*TreeNode
		for i := 0; i < len(b); i++ {
			l = append(l, b[i].Val)
			if b[i].Left != nil {
				t = append(t, b[i].Left)
			}
			if b[i].Right != nil {
				t = append(t, b[i].Right)
			}
		}
		r = append(r, l)
		b = t
	}

	s, e := 0, len(r)-1
	for s < e {
		r[s], r[e] = r[e], r[s]
		s++
		e--
	}

	return r
}

func main() {
	fmt.Println("a")
}
