package main

import (
	"fmt"
	"sort"
)

func minSubsequence(nums []int) []int {
	sort.Ints(nums)
	sum := 0
	for _, v := range nums {
		sum += v
	}
	accum := 0
	var ret []int
	for i := len(nums) - 1; i >= 0; i-- {
		ret = append(ret, nums[i])
		accum += nums[i]
		if accum > sum-accum {
			break
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
