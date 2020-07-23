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
	var result [][]int
	var recur func(root *TreeNode, sum int, accum []int)
	recur = func(root *TreeNode, sum int, accum []int) {
		if root.Left == nil && root.Right == nil {
			s := root.Val
			for i := 0; i < len(accum); i++ {
				s += accum[i]
			}
			if s == sum {
				result = append(result, append(accum, root.Val))
			}
			return
		}
		t := make([]int, len(accum)+1)
		copy(t, accum)
		t[len(t)-1] = root.Val
		if root.Left != nil {
			recur(root.Left, sum, t)
		}
		if root.Right != nil {
			recur(root.Right, sum, t)
		}
	}
	recur(root, sum, []int{})
	return result
}

func main() {
	fmt.Println("a")
}
