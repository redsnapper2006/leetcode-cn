package main

import (
	"fmt"
	"sort"
)

func minimumDifference(nums []int, k int) int {
	if k <= 1 {
		return 0
	}
	min := 1<<31 - 1
	sort.Ints(nums)
	for i := k - 1; i < len(nums); i++ {
		if min > nums[i]-nums[i-k+1] {
			min = nums[i] - nums[i-k+1]
		}
	}
	return min
}

func main() {
	fmt.Println()
}
