package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getMinimumDifference(root *TreeNode) int {
	ret := 1<<31 - 1
	var recur func(root, min, max *TreeNode)
	recur = func(root, min, max *TreeNode) {
		if root == nil {
			return
		}
		if min != nil {
			v := min.Val - root.Val
			if v < 0 {
				v = -v
			}
			if v < ret {
				ret = v
			}
		}
		if max != nil {
			v := max.Val - root.Val
			if v < 0 {
				v = -v
			}
			if v < ret {
				ret = v
			}
		}
		recur(root.Left, min, root)
		recur(root.Right, root, max)
	}

	recur(root, nil, nil)
	return ret
}

func main() {
	fmt.Println("a")
}
