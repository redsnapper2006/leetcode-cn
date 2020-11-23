package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func subtreeWithAllDeepest(root *TreeNode) *TreeNode {
	var recur func(root *TreeNode, depth int) (*TreeNode, int)
	recur = func(root *TreeNode, depth int) (*TreeNode, int) {
		if root.Left == nil && root.Right == nil {
			return root, depth
		}

		LD, RD := 0, 0
		var LP, RP *TreeNode = nil, nil
		if root.Left != nil {
			LP, LD = recur(root.Left, depth+1)
		}
		if root.Right != nil {
			RP, RD = recur(root.Right, depth+1)
		}
		if LD == RD {
			return root, LD
		}
		if LD > RD {
			return LP, LD
		}
		return RP, RD
	}

	ret, _ := recur(root, 0)
	return ret
}

func main() {
	fmt.Println()
}
