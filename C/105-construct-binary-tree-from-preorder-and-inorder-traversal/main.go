package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(inorder) == 0 {
		return nil
	}

	if len(inorder) == 1 {
		return &TreeNode{Val: inorder[0]}
	}

	rootInOrderIndex := -1
	for i := 0; i < len(inorder); i++ {
		if inorder[i] == preorder[0] {
			rootInOrderIndex = i
			break
		}
	}

	root := TreeNode{Val: preorder[0],
		Left:  buildTree(preorder[1:rootInOrderIndex+1], inorder[0:rootInOrderIndex]),
		Right: buildTree(preorder[rootInOrderIndex+1:], inorder[rootInOrderIndex+1:])}

	return &root
}
