package main

import "fmt"

func totalHammingDistance(nums []int) int {
	b := make([]int, 31)
	for i := 0; i < len(nums); i++ {
		for j := 0; j < 31; j++ {
			if (nums[i] & (1 << j)) != 0 {
				b[j]++
			}
		}
	}
	sum := 0
	for i := 0; i < len(b); i++ {
		sum += b[i] * (len(nums) - b[i])
	}
	return sum
}

func main() {
	fmt.Println(totalHammingDistance([]int{4, 14, 2}))
}
