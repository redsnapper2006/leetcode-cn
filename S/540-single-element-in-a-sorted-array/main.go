package main

import "fmt"

func singleNonDuplicateV2(nums []int) int {
	a := nums[0]
	for i := 1; i < len(nums); i++ {
		a = a ^ nums[i]
	}
	return a
}

func singleNonDuplicate(nums []int) int {
	s, e := 0, len(nums)-1
	for s < e {
		m := s + (e-s)/2
		if nums[m] == nums[m-1] {
			if (e-m)%2 == 0 {
				e = m - 2
			} else {
				s = m + 1
			}
		} else if nums[m] == nums[m+1] {
			if (e-m)%2 == 0 {
				s = m + 2
			} else {
				e = m - 1
			}
		} else {
			return nums[m]
		}
	}
	return nums[s]
}

func main() {
	fmt.Println("a")
}
