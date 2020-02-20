package main

import (
	"fmt"
	"sort"
)

func arrayPairSum(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	sort.Ints(nums)
	s, e := 0, len(nums)-2
	a := 0
	for s < e {
		a += nums[s] + nums[e]
		s += 2
		e -= 2
	}
	if s == e {
		a += nums[s]
	}
	return a
}

func main() {
	fmt.Println("a")
}
