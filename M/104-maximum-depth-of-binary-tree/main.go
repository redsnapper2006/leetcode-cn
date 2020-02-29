package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	l := maxDepth(root.Left)
	r := maxDepth(root.Right)

	if r > l {
		return r + 1
	}

	return l + 1
}

func maxDepthV2(root *TreeNode) int {
	if root == nil {
		return 0
	}

	c := 0
	var b []*TreeNode

	b = append(b, root)
	for len(b) > 0 {
		c++
		var t []*TreeNode
		for i := 0; i < len(b); i++ {
			if b[i].Left != nil {
				t = append(t, b[i].Left)
			}
			if b[i].Right != nil {
				t = append(t, b[i].Right)
			}
		}
		b = t
	}
	return c
}

func main() {

}
