package main

import (
	"fmt"
)

func removeDuplicates(nums []int) int {
	if len(nums) == 0 || len(nums) == 1 {
		return len(nums)
	}
	offset := 0
	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[offset] {
			offset++
			nums[offset] = nums[i]
		}
	}
	return offset + 1
}

func main() {
	fmt.Println(removeDuplicates([]int{1, 1, 2}))
	fmt.Println(removeDuplicates([]int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}))
}
