package main

import "fmt"

func canBeIncreasing(nums []int) bool {
	isFirst := true
	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[i-1] {
			continue
		} else {
			if i == len(nums)-1 {
				if !isFirst {
					return false
				}
			} else if i == 1 {
				if nums[i+1] <= nums[i] {
					return false
				} else {
					isFirst = false
				}
			} else {
				if isFirst && (nums[i+1] > nums[i-1] || (i >= 2 && nums[i] > nums[i-2])) {
					isFirst = false
				} else {
					return false
				}
			}
		}
	}
	return true
}

func main() {
	fmt.Println()
}
