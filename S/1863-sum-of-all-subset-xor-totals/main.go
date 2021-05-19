package main

import "fmt"

func subsetXORSum(nums []int) int {
	buf := []int{0, nums[0]}

	for i := 1; i < len(nums); i++ {
		size := len(buf)
		for j := 0; j < size; j++ {
			buf = append(buf, buf[j]^nums[i])
		}
	}
	sum := 0
	for i := 0; i < len(buf); i++ {
		sum += buf[i]
	}
	return sum
}

func main() {
	fmt.Println()
}
