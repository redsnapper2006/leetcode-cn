package main

import "fmt"

func shuffle(nums []int, n int) []int {
	var buf []int
	for i := 0; i < n; i++ {
		buf = append(buf, nums[i], nums[i+n])
	}
	return buf
}

func main() {
	fmt.Println("a")
}
