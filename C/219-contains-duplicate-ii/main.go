package main

import (
	"fmt"
)

func containsNearbyDuplicate(nums []int, k int) bool {
	if len(nums) == 0 || k <= 0 {
		return false
	}

	buf := make(map[int]int)
	for i := 0; i < k && i < len(nums); i++ {
		_, ok := buf[nums[i]]
		if ok {
			return true
		}
		buf[nums[i]]++
	}
	for i := k; i < len(nums); i++ {
		_, ok := buf[nums[i]]
		if ok {
			return true
		}
		buf[nums[i]]++
		delete(buf, nums[i-k])
	}
	return false
}

func main() {
	fmt.Println("a")
}
