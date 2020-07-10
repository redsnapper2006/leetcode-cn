package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pseudoPalindromicPaths(root *TreeNode) int {
	M := map[int]int{}

	c := 0
	var recur func(root *TreeNode)
	recur = func(root *TreeNode) {
		M[root.Val]++
		if root.Left == nil && root.Right == nil {
			odd := 0
			for _, v := range M {
				if v%2 == 1 {
					odd++
				}
			}
			if odd <= 1 {
				c++
			}
			M[root.Val]--
			return
		}

		if root.Left != nil {
			recur(root.Left)
		}

		if root.Right != nil {
			recur(root.Right)
		}
		M[root.Val]--
	}
	recur(root)
	return c
}

func main() {
	fmt.Println("a")
}
