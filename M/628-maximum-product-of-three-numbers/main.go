package main

import (
	"fmt"
	"sort"
)

func maximumProduct(nums []int) int {
	sort.Ints(nums)

	if nums[0] < 0 && nums[1] < 0 && nums[0]*nums[1]*nums[len(nums)-1] > nums[len(nums)-1]*nums[len(nums)-2]*nums[len(nums)-3] {
		return nums[0] * nums[1] * nums[len(nums)-1]
	}
	return nums[len(nums)-1] * nums[len(nums)-2] * nums[len(nums)-3]
}

func main() {
	fmt.Println("a")
}
