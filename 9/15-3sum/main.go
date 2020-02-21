package main

import (
	"fmt"
	"sort"
)

func threeSum(nums []int) [][]int {
	sort.Ints(nums)

	var buf [][]int
	for i := 0; i < len(nums); i++ {
		if nums[i] > 0 {
			break
		}
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		start, end := i+1, len(nums)-1
		for start < end {
			if start > i+1 && start < len(nums) && nums[start] == nums[start-1] {
				start++
				continue
			}
			if end < len(nums)-1 && end >= start && nums[end] == nums[end+1] {
				end--
				continue
			}
			if nums[i]+nums[start]+nums[end] > 0 {
				end--
			} else if nums[i]+nums[start]+nums[end] < 0 {
				start++
			} else {
				buf = append(buf, []int{nums[i], nums[start], nums[end]})
				start++
				end--
			}
		}
	}

	return buf
}

func main() {
	fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4}))
}
