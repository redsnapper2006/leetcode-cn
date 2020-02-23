package main

import (
	"fmt"
)

func containsNearbyAlmostDuplicateV2(nums []int, k int, t int) bool {
	if len(nums) == 0 {
		return false
	}
	if k == 0 {
		if t > 0 {
			return false
		} else if t == 0 {
			return false
		}
	}

	for i := 0; i <= k && i < len(nums); i++ {
		for j := i + 1; j <= k && j < len(nums); j++ {
			a := nums[i] - nums[j]
			if a < 0 {
				a *= -1
			}
			if a <= t {
				return true
			}
		}
	}

	for i := k + 1; i < len(nums); i++ {
		for j := 1; j <= k && j < len(nums); j++ {
			a := nums[i] - nums[i-j]
			if a < 0 {
				a *= -1
			}
			if a <= t {
				return true
			}
		}
	}
	return false
}

func containsNearbyAlmostDuplicate(nums []int, k int, t int) bool {
	if len(nums) == 0 || k == 0 {
		return false
	}

	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i]-nums[j] <= t && nums[i]-nums[j] >= t*-1 && i-j <= k && i-j >= k*-1 {
				return true
			}
		}
	}
	return false
}

func main() {
	fmt.Println(containsNearbyAlmostDuplicate([]int{1}, 1, 1))
}
