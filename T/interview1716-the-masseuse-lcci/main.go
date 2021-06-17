package main

import (
	"fmt"
)

func massage(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	if len(nums) == 1 {
		return nums[0]
	}
	if len(nums) == 2 {
		if nums[0] > nums[1] {
			return nums[0]
		} else {
			return nums[1]
		}
	}
	if len(nums) == 3 {
		if nums[0]+nums[2] > nums[1] {
			return nums[0] + nums[2]
		} else {
			return nums[1]
		}
	}

	buf := make([]int, len(nums))
	buf[0] = nums[0]
	buf[1] = nums[1]
	for i := 0; i < len(buf); i++ {
		if i+2 < len(buf) && buf[i]+nums[i+2] > buf[i+2] {
			buf[i+2] = buf[i] + nums[i+2]
		}
		if i+3 < len(buf) && buf[i]+nums[i+3] > buf[i+3] {
			buf[i+3] = buf[i] + nums[i+3]
		}
	}
	if buf[len(buf)-1] > buf[len(buf)-2] {
		return buf[len(buf)-1]
	} else {
		return buf[len(buf)-2]
	}
}

func main() {
	fmt.Println("a")
}
