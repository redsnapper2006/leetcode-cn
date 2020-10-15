package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func numComponents(head *ListNode, G []int) int {
	M := map[int]int{}
	for _, v := range G {
		M[v]++
	}

	p := head
	cnt := 0
	isIn := false
	for p != nil {
		_, ok := M[p.Val]
		if ok {
			if !isIn {
				isIn = true
			}
		} else {
			if isIn {
				cnt++
				isIn = false
			}
		}
		p = p.Next
	}
	if isIn {
		cnt++
	}
	return cnt
}

func main() {
	fmt.Println()
}
