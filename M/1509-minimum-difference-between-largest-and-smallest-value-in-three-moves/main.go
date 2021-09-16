package main

import (
	"fmt"
	"sort"
)

func minDifference(nums []int) int {
	if len(nums) <= 4 {

		return 0
	}

	sort.Ints(nums)
	min := nums[len(nums)-4] - nums[0]
	for i := 1; i <= 3; i++ {
		if nums[len(nums)-4+i]-nums[i] < min {
			min = nums[len(nums)-4+i] - nums[i]
		}
	}

	return min
}

func minDifferenceV2(nums []int) int {
	if len(nums) <= 4 {

		return 0
	}

	sort.Ints(nums)
	min := 1<<31 - 1
	var recur func(nums []int, s, e, times int)
	recur = func(nums []int, s, e, times int) {
		if times == 0 {
			if min > nums[e]-nums[s] {
				min = nums[e] - nums[s]
			}
			return
		}

		if s+1 < e {
			recur(nums, s+1, e, times-1)
		}
		if e-1 > s {
			recur(nums, s, e-1, times-1)
		}
	}

	return min
}

func main() {
	fmt.Println()
}
