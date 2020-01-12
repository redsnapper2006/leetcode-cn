package main

import (
	"fmt"
)

func maxProduct(nums []int) int {
	max := nums[0]
	for i := 0; i < len(nums); i++ {
		c := nums[i]
		if c > max {
			max = c
		}
		for j := i + 1; j < len(nums); j++ {
			c = c * nums[j]
			if c > max {
				max = c
			}
		}
	}
	return max
}

func main() {
	fmt.Println("a")
	fmt.Println(maxProduct([]int{-2, 0, -1}))
}
