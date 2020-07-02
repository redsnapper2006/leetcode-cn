package main

import "fmt"

func maxSubArray(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	if len(nums) == 1 {
		return nums[0]
	}
	buf := make([]int, len(nums))

	buf[0] = nums[0]
	max := buf[0]
	for i := 1; i < len(nums); i++ {
		if buf[i-1]+nums[i] > nums[i] {
			buf[i] = buf[i-1] + nums[i]
		} else {
			buf[i] = nums[i]
		}
		if buf[i] > max {
			max = buf[i]
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
