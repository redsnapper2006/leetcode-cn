package main

import "fmt"

func findMiddleIndex(nums []int) int {
	sum := make([]int, len(nums))

	s := 0
	for i, v := range nums {
		s += v
		sum[i] = s
	}

	for i := 0; i < len(sum); i++ {
		left, right := 0, 0
		if i > 0 {
			left = sum[i-1]
		}
		if i < len(sum)-1 {
			right = s - sum[i]
		}
		if left == right {
			return i
		}
	}
	return -1
}

func main() {
	fmt.Println()
}
