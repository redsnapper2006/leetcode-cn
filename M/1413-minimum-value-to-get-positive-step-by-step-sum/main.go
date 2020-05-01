package main

import "fmt"

func minStartValue(nums []int) int {
	min := 0
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		if sum < min {
			min = sum
		}
	}
	return -min + 1
}

func main() {
	fmt.Println("a")
}
