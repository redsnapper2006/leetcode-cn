package main

import (
	"fmt"
)

func rob(nums []int) int {

	size := len(nums)
	if size == 0 {
		return 0
	}
	if size == 1 {
		return nums[0]
	}
	if size == 2 {
		if nums[0] > nums[1] {
			return nums[0]
		} else {
			return nums[1]
		}
	}
	buf := make([]int, size)
	buf[0], buf[1] = nums[0], nums[1]
	for i := 0; i < size; i++ {
		for j := 2; i+j < size; j++ {
			if buf[i+j] < buf[i]+nums[i+j] {
				buf[i+j] = buf[i] + nums[i+j]
			}
		}
	}

	if buf[len(buf)-1] > buf[len(buf)-2] {
		return buf[len(buf)-1]
	} else {
		return buf[len(buf)-2]
	}
}

func main() {
	fmt.Println(rob([]int{1, 2, 3, 1}))
	fmt.Println(rob([]int{2, 7, 9, 3, 1}))
}
