package main

import "fmt"

func search(nums []int, target int) int {
	ret := 0
	s, e := 0, len(nums)-1
	for s <= e {
		m := s + (e-s)/2
		if nums[m] > target {
			e = m - 1
		} else if nums[m] < target {
			s = m + 1
		} else {
			t := m - 1
			for t >= 0 && nums[t] == target {
				ret++
				t--
			}
			t = m + 1
			for t < len(nums) && nums[t] == target {
				ret++
				t++
			}
			return ret + 1
		}
	}
	return 0
}

func main() {
	fmt.Println("a")
}
