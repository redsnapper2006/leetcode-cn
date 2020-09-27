package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findTarget(root *TreeNode, k int) bool {
	M := map[int]int{}
	var recur func(r *TreeNode, k int) bool
	recur = func(r *TreeNode, k int) bool {
		if r == nil {
			return false
		}
		_, ok := M[r.Val]
		if ok {
			return true
		}
		M[k-r.Val]++
		return recur(r.Left, k) || recur(r.Right, k)
	}
	return recur(root, k)
}

func main() {
	fmt.Println("a")
}
