package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func allPossibleFBT(N int) []*TreeNode {
	if N == 1 {
		return []*TreeNode{&TreeNode{Val: 0, Left: nil, Right: nil}}
	}

	var ret []*TreeNode
	cnt := N - 1
	for i := 1; i < cnt; i = i + 2 {
		left := allPossibleFBT(i)
		right := allPossibleFBT(cnt - i)

		for i := 0; i < len(left); i++ {
			for j := 0; j < len(right); j++ {
				ret = append(ret, &TreeNode{Val: 0, Left: left[i], Right: right[j]})
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
