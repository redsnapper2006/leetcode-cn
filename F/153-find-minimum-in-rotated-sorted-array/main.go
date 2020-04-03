package main

import "fmt"

func findMin(nums []int) int {
	s, e := 0, len(nums)-1

	for s < e {
		m := s + (e-s)/2

		if nums[m] >= nums[s] && nums[m] > nums[e] {
			s = m + 1
		} else if nums[m] <= nums[s] && nums[m] < nums[e] {
			e = m
		} else if nums[m] >= nums[s] && nums[m] < nums[e] {
			break
		}
	}
	return nums[s]
}

func main() {
	fmt.Println(findMin([]int{4, 5, 6, 7, 0, 1, 2}))
}
