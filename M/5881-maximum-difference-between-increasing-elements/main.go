package main

import "fmt"

func maximumDifference(nums []int) int {
	ret := -1

	min := nums[0]
	for i := 1; i < len(nums); i++ {
		if nums[i] <= min {
			min = nums[i]
		} else if nums[i]-min > ret {
			ret = nums[i] - min
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
