package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rangeSumBST(root *TreeNode, L int, R int) int {
	var recur func(root *TreeNode, L, R int, result *int)
	recur = func(root *TreeNode, L, R int, result *int) {
		if root == nil {
			return
		}
		if root.Val >= L && root.Val <= R {
			*result += root.Val
			recur(root.Left, L, R, result)
			recur(root.Right, L, R, result)
		}
		if root.Val < L {
			recur(root.Right, L, R, result)
		}
		if root.Val > R {
			recur(root.Left, L, R, result)
		}
	}
	result := 0
	recur(root, L, R, &result)
	return result
}

func main() {
	fmt.Println("a")
}
