package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findTarget(root *TreeNode, k int) bool {

	var midrecur func(root *TreeNode) []int
	midrecur = func(root *TreeNode) []int {
		if root == nil {
			return nil
		}
		left := midrecur(root.Left)
		right := midrecur(root.Right)
		buf := make([]int, len(left)+len(right)+1)
		copy(buf, left)
		buf[len(left)] = root.Val
		copy(buf[len(left)+1:], right)
		return buf
	}
	arr := midrecur(root)
	s, e := 0, len(arr)-1
	for s < e {
		if arr[s]+arr[e] == k {
			return true
		} else if arr[s]+arr[e] > k {
			e--
		} else {
			s++
		}
	}
	return false
}

func main() {
	fmt.Println()
}
