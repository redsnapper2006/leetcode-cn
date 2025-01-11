package main

import "fmt"

func missingNumber(nums []int) int {
	s, e := 0, len(nums)-1
	for s <= e {
		m := s + (e-s)/2
		if nums[m] == m {
			s = m + 1
		} else if nums[m] > m {
			e = m - 1
		}
	}
	return s
}

func main() {
	fmt.Println("a")
}
