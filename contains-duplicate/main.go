package main

import (
	"fmt"
	"sort"
)

func containsDuplicate(nums []int) bool {
	if len(nums) <= 1 {
		return false
	}
	sort.Ints(nums)
	t := nums[0]
	for i := 1; i < len(nums); i++ {
		if t == nums[i] {
			return true
		}
		t = nums[i]
	}
	return false
}

func main() {

	fmt.Println(containsDuplicate([]int{1, 2, 3, 1}))
	fmt.Println(containsDuplicate([]int{1, 2, 3, 4}))
	fmt.Println(containsDuplicate([]int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}))
}
