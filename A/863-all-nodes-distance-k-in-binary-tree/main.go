package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func distanceK(root *TreeNode, target *TreeNode, K int) []int {
	var ret []int
	var deep func(root *TreeNode, K int, ret *[]int)
	deep = func(root *TreeNode, K int, ret *[]int) {
		if root == nil {
			return
		}
		// fmt.Println(root.Val, K)
		if K == 0 {
			*ret = append(*ret, root.Val)
			return
		}
		deep(root.Left, K-1, ret)
		deep(root.Right, K-1, ret)
	}
	var recur func(root *TreeNode, target *TreeNode, K int, ret *[]int) int
	recur = func(root *TreeNode, target *TreeNode, K int, ret *[]int) int {
		if root == nil {
			return -1
		}
		if root.Val == target.Val {
			if K == 0 {
				*ret = append(*ret, root.Val)
			} else {
				deep(root.Left, K-1, ret)
				deep(root.Right, K-1, ret)
			}
			return 1
		}

		L := recur(root.Left, target, K, ret)
		// fmt.Println(root.Val, L)
		R := recur(root.Right, target, K, ret)
		if L != -1 {
			if K > L {
				deep(root.Right, K-L-1, ret)
			} else if K == L {
				*ret = append(*ret, root.Val)
			}
			return L + 1
		}
		if R != -1 {
			if K > R {
				deep(root.Left, K-R-1, ret)
			} else if K == R {
				*ret = append(*ret, root.Val)
			}
			return R + 1
		}
		return -1
	}

	recur(root, target, K, &ret)
	return ret
}

func main() {
	fmt.Println()
}
