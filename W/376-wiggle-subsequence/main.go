package main

import "fmt"

func wiggleMaxLength(nums []int) int {
	if len(nums) <= 1 {
		return len(nums)
	}
	count := 0
	trend := 0
	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[i-1] {
			if trend == 0 || trend == 2 {
				count++
				trend = 1
			}
		} else if nums[i] < nums[i-1] {
			if trend == 0 || trend == 1 {
				count++
				trend = 2
			}
		}
	}
	return count + 1
}

func main() {
	fmt.Println()
}
