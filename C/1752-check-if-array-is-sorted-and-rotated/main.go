package main

import "fmt"

func check(nums []int) bool {
	idx := -1

	for i := 1; i < len(nums); i++ {
		if nums[i] < nums[i-1] {
			idx = i
			break
		}
	}
	if idx == -1 {
		return true
	}

	for i := 1; i < len(nums); i++ {
		ii := (idx + i) % len(nums)
		iii := (idx + i - 1) % len(nums)
		if nums[ii] < nums[iii] {
			return false
		}
	}

	return true
}

func main() {
	fmt.Println()
}
