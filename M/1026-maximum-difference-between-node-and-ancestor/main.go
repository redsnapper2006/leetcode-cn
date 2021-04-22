package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxAncestorDiff(root *TreeNode) int {
	if root == nil {
		return 0
	}
	var recur func(root *TreeNode, min, max int, ret *int)
	recur = func(root *TreeNode, min, max int, ret *int) {
		if root == nil {
			return
		}

		v := root.Val
		if v < min {
			min = v
		}
		if v > max {
			max = v
		}
		if *ret < max-min {
			*ret = max - min
		}
		recur(root.Left, min, max, ret)
		recur(root.Right, min, max, ret)
	}
	ret := 0
	min, max := root.Val, root.Val
	recur(root, min, max, &ret)
	return ret
}

func main() {
	fmt.Println(maxAncestorDiff(nil))
}
