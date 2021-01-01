package main

import "fmt"

func checkSubarraySum(nums []int, k int) bool {
	if len(nums) <= 1 {
		return false
	}

	M := map[int]int{}
	M[0] = -1
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		idx := sum
		if k != 0 {
			idx = sum % k
		}
		p, ok := M[idx]
		if !ok {
			M[idx] = i
		} else if i-p >= 2 {
			return true
		}
	}

	return false
}

func main() {
	fmt.Println()
}
