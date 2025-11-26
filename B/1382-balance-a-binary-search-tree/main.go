package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func balanceBST(root *TreeNode) *TreeNode {
	buf := []int{}

	var ctor func(root *TreeNode, buf *[]int)
	ctor = func(root *TreeNode, buf *[]int) {
		if root == nil {
			return
		}
		if root.Left != nil {
			ctor(root.Left, buf)
		}
		*buf = append(*buf, root.Val)
		if root.Right != nil {
			ctor(root.Right, buf)
		}
	}

	ctor(root, &buf)

	var build func(buf []int, s, e int) *TreeNode
	build = func(buf []int, s, e int) *TreeNode {
		if s > e {
			return nil
		}
		m := s + (e-s)/2
		tn := TreeNode{Val: buf[m]}
		tn.Left = build(buf, s, m-1)
		tn.Right = build(buf, m+1, e)
		return &tn
	}

	return build(buf, 0, len(buf)-1)
}

func main() {
	tn := TreeNode{Val: 2, Left: &TreeNode{Val: 1}, Right: &TreeNode{Val: 3, Right: &TreeNode{Val: 4}}}
	balanceBST(&tn)
}
