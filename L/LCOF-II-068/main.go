package main

import "fmt"

func searchInsert(nums []int, target int) int {
	s, e := 0, len(nums)-1
	for s <= e {
		m := s + (e-s)/2
		if nums[m] > target {
			e = m - 1
		} else if nums[m] < target {
			s = m + 1
		} else {
			return m
		}
	}
	return s
}

func main() {
	fmt.Println()
}
