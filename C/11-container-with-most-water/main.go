package main

import (
	"fmt"
)

func maxArea(height []int) int {
	start, end := 0, len(height)-1
	max := 0

	for start < end {
		base := start
		isStart := true
		if height[base] > height[end] {
			base = end
			isStart = false
		}
		fmt.Println(base, start, end)
		if height[base]*(end-start) > max {
			max = height[base] * (end - start)
		}

		if isStart {
			start++
		} else {
			end--
		}
	}
	return max
}

func main() {
	fmt.Println(maxArea([]int{1, 8, 6, 2, 5, 4, 8, 3, 7}))
}
