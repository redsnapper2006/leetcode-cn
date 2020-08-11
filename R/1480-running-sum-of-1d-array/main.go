package main

import "fmt"

func runningSum(nums []int) []int {
	var buf []int
	b := 0
	for i := 0; i < len(nums); i++ {
		b += nums[i]
		buf = append(buf, b)
	}
	return buf
}

func main() {
	fmt.Println("a")
}
