package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func nodesBetweenCriticalPoints(head *ListNode) []int {
	minIdx, maxIdx := 1000000, -1
	prevIdx := -1
	currIdx := -1
	p := head
	var pp *ListNode

	min := 1000000
	for p != nil {
		currIdx++
		if pp != nil && p.Next != nil {
			if p.Val < pp.Val && p.Val < p.Next.Val || p.Val > pp.Val && p.Val > p.Next.Val {
				if prevIdx > -1 && currIdx-prevIdx < min {
					min = currIdx - prevIdx
				}
				prevIdx = currIdx
				if currIdx < minIdx {
					minIdx = currIdx
				}
				if currIdx > maxIdx {
					maxIdx = currIdx
				}
			}
		}
		pp = p
		p = p.Next
	}
	// fmt.Println(minIdx,maxIdx)
	if maxIdx == minIdx || minIdx == 1000000 && maxIdx == -1 {
		return []int{-1, -1}
	}
	return []int{min, maxIdx - minIdx}
}

func main() {
	fmt.Println()
}
