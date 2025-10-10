package main

import (
	"fmt"
	"sort"
)

func minPairSum(nums []int) int {
	sort.Ints(nums)
	min := nums[0] + nums[len(nums)-1]
	for i := 1; i < len(nums)/2; i++ {
		if min < nums[i]+nums[len(nums)-1-i] {
			min = nums[i] + nums[len(nums)-1-i]
		}
	}
	return min
}

func main() {
	fmt.Println()
}
