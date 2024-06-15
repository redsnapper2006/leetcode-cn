package main

import "sort"

func maximumBeauty(nums []int, k int) int {
	sort.Ints(nums)

	start, end := 0, 0
	ans := 0
	for end < len(nums) {
		if nums[start]+k*2 >= nums[end] {
			end++
			continue
		}
		if ans < end-start {
			ans = end - start
		}
		for nums[start]+k*2 < nums[end] {
			start++
		}
	}
	if ans < end-start {
		ans = end - start
	}
	return ans
}
