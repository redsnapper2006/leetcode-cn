package main

import "fmt"

func minOperations(nums []int) int {
	cnt := 0
	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[i-1] {
			continue
		}
		cnt += nums[i-1] + 1 - nums[i]
		nums[i] = nums[i-1] + 1
	}
	return cnt
}

func main() {
	fmt.Println(minOperations([]int{1, 2, 3, 4}))
}
