package main

import (
	"fmt"
)

func maxProduct(nums []int) int {
	buf := make([][]int, 2)
	for i := 0; i < 2; i++ {
		buf[i] = make([]int, len(nums))
	}
	buf[0][0] = nums[0]
	buf[1][0] = nums[0]
	for i := 1; i < len(nums); i++ {
		if nums[i] >= 0 {
			t := buf[0][i-1] * nums[i]
			if t < nums[i] {
				t = nums[i]
			}
			buf[0][i] = t
			t = buf[1][i-1] * nums[i]
			if t > nums[i] {
				t = nums[i]
			}
			buf[1][i] = t
		} else {
			t := buf[1][i-1] * nums[i]
			if t < nums[i] {
				t = nums[i]
			}
			buf[0][i] = t
			t = buf[0][i-1] * nums[i]
			if t > nums[i] {
				t = nums[i]
			}
			buf[1][i] = t
		}
	}
	max := -1 << 31
	for i := 0; i < len(buf[0]); i++ {
		if max < buf[0][i] {
			max = buf[0][i]
		}
		if max < buf[1][i] {
			max = buf[1][i]
		}
	}

	return max
}

func maxProductV2(nums []int) int {
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
	fmt.Println(maxProduct([]int{2, -5, -2, -4, 3}))
}
