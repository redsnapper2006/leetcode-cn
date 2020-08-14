package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pathSum(root *TreeNode, sum int) [][]int {
	if root == nil {
		return nil
	}

	var ret [][]int
	var recur func(root *TreeNode, sum int, accum []int)
	recur = func(root *TreeNode, sum int, accum []int) {
		if root.Left == nil && root.Right == nil {
			ss := 0
			for i := 0; i < len(accum); i++ {
				ss += accum[i]
			}
			if ss+root.Val == sum {
				t := make([]int, len(accum)+1)
				copy(t, accum)
				t[len(accum)] = root.Val
				ret = append(ret, t)
			}
			return
		}

		b := make([]int, len(accum)+1)
		copy(b, accum)
		b[len(accum)] = root.Val
		if root.Left != nil {
			recur(root.Left, sum, b)
		}
		if root.Right != nil {
			recur(root.Right, sum, b)
		}
	}
	recur(root, sum, []int{})
	return ret
}

func main() {
	fmt.Println("a")
}
