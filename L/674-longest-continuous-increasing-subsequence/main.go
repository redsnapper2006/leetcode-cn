package main

import "fmt"

func findLengthOfLCIS(nums []int) int {
	if len(nums) <= 1 {
		return len(nums)
	}
	max := 0
	c := 1
	b := nums[0]
	for i := 1; i < len(nums); i++ {
		if nums[i] > b {
			c++
		} else {
			if c > max {
				max = c
			}
			c = 1
		}
		b = nums[i]
	}
	if c > max {
		max = c
	}
	return max
}

func main() {
	fmt.Println("a")
}
