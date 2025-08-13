package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func flipMatchVoyage(root *TreeNode, voyage []int) []int {

	var recur func(root *TreeNode, voyage []int, index *int, ans *[]int)
	recur = func(root *TreeNode, voyage []int, index *int, ans *[]int) {
		if root.Val != voyage[*index] {
			*ans = []int{-1}
			return
		}
		*index++
		if root.Left != nil {
			if root.Left.Val == voyage[*index] {
				recur(root.Left, voyage, index, ans)
				if len(*ans) > 0 && (*ans)[0] == -1 {
					return
				}
			} else if root.Right == nil {
				*ans = []int{-1}
			} else {
				*ans = append(*ans, root.Val)
				recur(root.Right, voyage, index, ans)
				if len(*ans) > 0 && (*ans)[0] == -1 {
					return
				}
			}
		}
		if root.Right != nil {
			if root.Right.Val == voyage[*index] {
				recur(root.Right, voyage, index, ans)
				if len(*ans) > 0 && (*ans)[0] == -1 {
					return
				}
			} else if root.Left == nil {
				*ans = []int{-1}
			} else {
				recur(root.Left, voyage, index, ans)
				if len(*ans) > 0 && (*ans)[0] == -1 {
					return
				}
			}
		}
	}

	index := 0
	ans := []int{}

	recur(root, voyage, &index, &ans)

	return ans
}
