package main

import "fmt"

func findRepeatNumber(nums []int) int {
	buf := make([]bool, len(nums))
	for i := 0; i < len(buf); i++ {
		buf[i] = false
	}
	for i := 0; i < len(nums); i++ {
		if buf[nums[i]] == true {
			return nums[i]
		}
		buf[nums[i]] = true
	}
	return -1
}

func main() {
	fmt.Println("a")
}
