package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func nextLargerNodes(head *ListNode) []int {
	var stack []int
	var sidx []int
	var ret []int
	p := head
	for p != nil {
		ret = append(ret, 0)
		if len(stack) == 0 {
			stack = append(stack, p.Val)
			sidx = append(sidx, len(ret)-1)
		} else {
			idx := -1
			for j := len(stack) - 1; j >= 0; j-- {
				if stack[j] < p.Val {
					ret[sidx[j]] = p.Val
				} else {
					idx = j
					break
				}
			}

			stack = stack[0 : idx+1]
			sidx = sidx[0 : idx+1]

			stack = append(stack, p.Val)
			sidx = append(sidx, len(ret)-1)
		}
		p = p.Next
	}

	return ret
}

func main() {
	fmt.Println()
}
