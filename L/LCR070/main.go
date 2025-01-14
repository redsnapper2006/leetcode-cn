package main

import "fmt"

func singleNonDuplicate(nums []int) int {
	if len(nums) == 0 {
		return -1
	}
	if len(nums) == 1 {
		return nums[0]
	}
	if len(nums) == 2 {
		return -1
	}
	s, e := 0, len(nums)-1
	m := s + (e-s)/2
	// fmt.Println(s,e,m,nums)
	if nums[m] == nums[m+1] {
		right := singleNonDuplicate(nums[m+2:])
		left := singleNonDuplicate(nums[0:m])
		if left != -1 {
			return left
		}
		if right != -1 {
			return right
		}
	} else if nums[m] == nums[m-1] {
		right := singleNonDuplicate(nums[m+1:])
		left := singleNonDuplicate(nums[0 : m-1])
		// fmt.Println(right,left)
		if left != -1 {
			return left
		}
		if right != -1 {
			return right
		}
	} else {
		return nums[m]
	}
	return -1
}

func main() {
	fmt.Println()
}
