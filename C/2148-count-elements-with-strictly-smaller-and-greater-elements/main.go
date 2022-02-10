package main

import (
	"fmt"
	"sort"
)

func countElements(nums []int) int {
	sort.Ints(nums)

	s := len(nums)
	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[0] {
			s = i
			break
		}
	}
	e := 0
	for i := len(nums) - 2; i >= 0; i-- {
		if nums[i] < nums[len(nums)-1] {
			e = i
			break
		}
	}
	if e < s {
		return 0
	}
	return e - s + 1
}

func main() {
	fmt.Println()
}
