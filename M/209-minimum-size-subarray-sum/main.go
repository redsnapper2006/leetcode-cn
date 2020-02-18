package main

import (
	"fmt"
)

func minSubArrayLen(s int, nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	sumBuf := make([]int, len(nums))
	accum := 0
	maxLen := 0
	for i, v := range nums {
		accum += v
		sumBuf[i] = accum
	}
	for i, v := range sumBuf {
		if v >= s {
			maxLen = i + 1
			break
		}
	}
	if maxLen == 0 {
		return 0
	}
	endLen := maxLen
	startLen := 1
	for startLen < endLen {
		l := startLen + (endLen-startLen)/2
		isFound := false
		for i := len(nums) - 1; i > l; i-- {
			if (sumBuf[i] - sumBuf[i-l]) >= s {
				isFound = true
				break
			}
		}
		if isFound {
			endLen = l
		} else {
			startLen = l + 1
		}
	}
	return endLen
}

func main() {
	fmt.Println(minSubArrayLen(7, []int{2, 3, 1, 2, 4, 3}))
	fmt.Println(minSubArrayLen(11, []int{1, 2, 3, 4, 5}))
}
