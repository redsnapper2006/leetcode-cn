package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pathSum(root *TreeNode, sum int) int {
	ret := 0
	var recur func(root *TreeNode, container []int)
	recur = func(root *TreeNode, container []int) {
		if root == nil {
			return
		}

		accum := root.Val
		if accum == sum {
			ret++
		}
		for i := len(container) - 1; i >= 0; i-- {
			accum += container[i]
			if accum == sum {
				ret++
			}
		}
		recur(root.Left, append(container, root.Val))
		recur(root.Right, append(container, root.Val))
	}
	recur(root, []int{})
	return ret
}

func main() {
	fmt.Println("a")
}
