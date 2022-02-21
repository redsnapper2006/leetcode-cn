package main

import "fmt"

func sumOfBeauties(nums []int) int {
	lBuf, rBuf := make([]int, len(nums)), make([]int, len(nums))
	max := -1
	for i := 0; i < len(nums); i++ {
		if nums[i] > max {
			max = nums[i]
		}
		lBuf[i] = max
	}
	min := 1000000
	for i := len(nums) - 1; i >= 0; i-- {
		if nums[i] < min {
			min = nums[i]
		}
		rBuf[i] = min
	}

	ret := 0
	for i := 0; i < len(nums); i++ {
		if i == 0 || i == len(nums)-1 {
			continue
		}
		if nums[i] > lBuf[i-1] && nums[i] < rBuf[i+1] {
			ret += 2
		} else if nums[i] > nums[i-1] && nums[i] < nums[i+1] {
			ret++
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
