package main

import "fmt"

func buildArray(nums []int) []int {
	ret := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		ret[i] = nums[nums[i]]
	}
	return ret
}

func main() {
	fmt.Println()
}
