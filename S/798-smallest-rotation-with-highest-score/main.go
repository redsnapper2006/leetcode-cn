package main

import "fmt"

func bestRotation(nums []int) int {
	buf := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		offset := i - nums[i]
		if offset >= 0 {
			buf[0]++
			if offset+1 < len(nums) {
				buf[offset+1]--
			}
			if i+1 < len(nums) {
				buf[i+1]++
			}
		} else {
			if i+1 < len(nums) {
				buf[i+1]++
			}
			if len(nums)-nums[i]+i+1 < len(nums) {
				buf[len(nums)-nums[i]+i+1]--
			}
		}
	}

	max := 0
	idx := -1

	sum := 0
	for i := 0; i < len(buf); i++ {
		sum += buf[i]
		buf[i] = sum
		if sum > max {
			max = sum
			idx = i
		}
	}
	return idx
}

func main() {
	fmt.Println()
}
