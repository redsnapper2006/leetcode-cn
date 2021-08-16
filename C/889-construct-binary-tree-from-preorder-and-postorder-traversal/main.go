package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func constructFromPrePost(preorder []int, postorder []int) *TreeNode {
	var recur func(preorder []int, postorder []int) *TreeNode

	recur = func(preorder []int, postorder []int) *TreeNode {
		if len(preorder) == 0 {
			return nil
		}
		root := TreeNode{Val: preorder[0]}
		if len(preorder) > 1 {
			leftVal := preorder[1]
			idx := -1
			for j := 0; j < len(postorder); j++ {
				if postorder[j] == leftVal {
					idx = j
					break
				}
			}
			root.Left = recur(preorder[1:idx+2], postorder[0:idx+1])
			if idx+2 < len(preorder) {
				root.Right = recur(preorder[idx+2:], postorder[idx+1:])
			}
		}

		return &root
	}
	return recur(preorder, postorder)
}

func main() {
	fmt.Println()
}
