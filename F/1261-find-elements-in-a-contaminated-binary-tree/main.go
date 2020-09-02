package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type FindElements struct {
	H *TreeNode
	M map[int]int
}

func Constructor(root *TreeNode) FindElements {
	m := map[int]int{}
	var recur func(r *TreeNode, V int)
	recur = func(r *TreeNode, V int) {
		r.Val = V
		m[V]++
		if r.Left == nil && r.Right == nil {
			return
		}
		if r.Left != nil {
			recur(r.Left, 2*V+1)
		}
		if r.Right != nil {
			recur(r.Right, 2*V+2)
		}
	}
	recur(root, 0)
	return FindElements{H: root, M: m}
}

func (this *FindElements) Find(target int) bool {
	return this.M[target] != 0
}

func main() {
	fmt.Println("a")
}
