package main

import (
	"fmt"
	"sort"
)

func specialArray(nums []int) int {
	sort.Ints(nums)

	for i := len(nums); i > 0; i-- {
		s, e := 0, len(nums)-1
		for s <= e {
			m := s + (e-s)/2
			if nums[m] < i {
				s = m + 1
			} else {
				e = m - 1
			}
		}
		if s >= len(nums) {
			continue
		}
		if len(nums)-s == i {
			return i
		}
	}
	return -1
}

func main() {
	fmt.Println()
}
