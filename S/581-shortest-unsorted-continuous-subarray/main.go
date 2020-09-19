package main

import (
	"fmt"
	"sort"
)

func findUnsortedSubarray(nums []int) int {
	buf := make([]int, len(nums))
	copy(buf, nums)
	sort.Ints(buf)
	s, e := -1, -1
	for i := range nums {
		if buf[i] == nums[i] {
			continue
		}
		s = i
		break
	}
	for i := len(nums) - 1; i >= 0; i-- {
		if buf[i] == nums[i] {
			continue
		}
		e = i
		break
	}
	return e - s + 1
}

func main() {
	fmt.Println("a")
}
