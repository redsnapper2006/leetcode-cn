package main

import (
	"fmt"
)

func minMoves(nums []int) int {
	min := nums[0]
	for i := 1; i < len(nums); i++ {
		if min > nums[i] {
			min = nums[i]
		}
	}
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i] - min
	}
	return sum
}

func main() {
	fmt.Println("a")
}
