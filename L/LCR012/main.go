package main

import "fmt"

func pivotIndex(nums []int) int {
	buf := make([]int, len(nums))
	sum := 0
	for i, v := range nums {
		sum += v
		buf[i] = sum
	}

	for i := 0; i < len(nums); i++ {
		d := 0
		if i > 0 {
			d = buf[i] - nums[i]
		}
		if d == sum-buf[i] {
			return i
		}
	}
	return -1
}

func main() {
	fmt.Println()
}
