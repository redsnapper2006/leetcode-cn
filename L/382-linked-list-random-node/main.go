package main

import (
	"fmt"
	"math/rand"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

type Solution struct {
	H *ListNode
}

func Constructor(head *ListNode) Solution {
	return Solution{H: head}
}

func (this *Solution) GetRandom() int {
	cnt := 1
	p := this.H
	var ans int
	for p != nil {
		if rand.Intn(cnt) == 0 {
			ans = p.Val
		}
		p = p.Next
		cnt++
	}
	return ans
}

func main() {
	fmt.Println()
}
