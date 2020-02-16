package main

import (
	"fmt"
)

func lengthOfLIS(nums []int) int {
	if len(nums) <=1 {
		return len(nums)
	}
	buf := make([]int, len(nums))

	for i := len(nums) - 1; i >= 0; i-- {
		candi := nums[i]
		max := 0
		for j := i + 1; j < len(nums); j++ {
			if candi < nums[j] && max < buf[j]+1 {
				max = buf[j] + 1
			}
		}
		buf[i] = max
	}
	r := 0
	for i:=0;i<len(buf);i++ {
		if r < buf[i] {
			r = buf[i]
		}
	}
	return r+1
}

func main() {
	fmt.Println(lengthOfLIS([]int{10, 9, 2, 5, 3, 7, 101, 18}))
}
