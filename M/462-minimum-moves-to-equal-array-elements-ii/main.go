package main

import (
	"fmt"
	"sort"
)

func minMoves2(nums []int) int {
	sort.Ints(nums)
	s, e := 0, len(nums)-1
	steps := 0
	for s < e {
		steps += nums[e] - nums[s]
		s++
		e--
	}
	return steps
}

func main() {
	fmt.Println("a")
}
