package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func leafSimilar(root1 *TreeNode, root2 *TreeNode) bool {
	var recur func(r *TreeNode, arr *[]int)
	recur = func(r *TreeNode, arr *[]int) {
		if r.Left == nil && r.Right == nil {
			*arr = append(*arr, r.Val)
			return
		}
		if r.Left != nil {
			recur(r.Left, arr)
		}
		if r.Right != nil {
			recur(r.Right, arr)
		}
	}

	var b1, b2 []int
	recur(root1, &b1)
	recur(root2, &b2)
	if len(b1) != len(b2) {
		return false
	}
	for i := 0; i < len(b1); i++ {
		if b1[i] != b2[i] {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
