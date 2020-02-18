package main

import (
	"fmt"
)

func removeDuplicates(nums []int) int {
	if len(nums) <= 2 {
		return len(nums)
	}
	offset := 0
	c := 1
	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[offset] {
			offset++
			nums[offset] = nums[i]
			c = 1
		} else if c == 1 {
			c++
			offset++
			nums[offset] = nums[i]
		}
	}

	return offset + 1
}

func main() {
	fmt.Println(removeDuplicates(([]int{1, 1, 1, 2, 2, 3})))
}
