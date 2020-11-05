package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func bstFromPreorder(preorder []int) *TreeNode {
	if len(preorder) == 0 {
		return nil
	}
	if len(preorder) == 1 {
		return &TreeNode{Val: preorder[0]}
	}
	base := preorder[0]
	idx := len(preorder)
	for i, v := range preorder {
		if v > base {
			idx = i
			break
		}
	}
	return &TreeNode{Val: base, Left: bstFromPreorder(preorder[1:idx]), Right: bstFromPreorder(preorder[idx:])}
}

func main() {
	fmt.Println("a")
}
