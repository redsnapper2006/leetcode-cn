package main

import "fmt"

func search(nums []int, target int) bool {
	if len(nums) == 0 {
		return false
	}

	if len(nums) == 1 {
		return nums[0] == target
	}
	if len(nums) == 2 {
		return nums[0] == target || nums[1] == target
	}

	s, e := 0, len(nums)
	m := s + (e-s)/2
	if nums[m] == target {
		return true
	}
	return search(nums[0:m], target) || search(nums[m+1:], target)
}

func main() {
	fmt.Println("a")
}
