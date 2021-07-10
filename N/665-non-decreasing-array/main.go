package main

import "fmt"

func checkPossibility(nums []int) bool {
	idx := -1
	isFirst := true
	for i := 1; i < len(nums); i++ {
		if nums[i] < nums[i-1] {
			if isFirst {
				idx = i
				isFirst = false
			} else {
				return false
			}
		}
	}
	if isFirst {
		return true
	}

	if idx == 1 || idx == len(nums)-1 {
		return true
	}
	if nums[idx+1] >= nums[idx-1] || nums[idx] >= nums[idx-2] {
		return true
	}
	return false
}

func main() {
	fmt.Println("a")
}
