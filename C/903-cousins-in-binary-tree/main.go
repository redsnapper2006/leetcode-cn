package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isCousins(root *TreeNode, x int, y int) bool {
	MV := map[int]int{}
	MP := map[int]*TreeNode{}

	var recur func(r, p *TreeNode, depth int)
	recur = func(r, p *TreeNode, depth int) {
		if r == nil {
			return
		}
		MV[r.Val] = depth
		MP[r.Val] = p
		recur(r.Left, r, depth+1)
		recur(r.Right, r, depth+1)
	}
	recur(root, nil, 0)
	return MV[x] == MV[y] && MP[x] != MP[y]
}

func main() {
	fmt.Println("a")
}
