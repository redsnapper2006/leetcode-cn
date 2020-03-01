package main

import "fmt"

func canJump(nums []int) bool {

	k := 0
	for i := 0; i < len(nums); i++ {
		if i > k {
			return false
		}
		if nums[i]+i > k {
			k = nums[i] + i
		}
	}
	return true
}

func canJumpV2(nums []int) bool {
	dp := make([]int, len(nums))
	dp[len(nums)-1] = 1

	for i := len(nums) - 2; i >= 0; i-- {
		isCan := false
		for j := 1; i+j < len(nums) && j <= nums[i]; j++ {
			if dp[i+j] == 1 {
				isCan = true
			}
		}
		if isCan {
			dp[i] = 1
		}
	}
	return dp[0] == 1
}

func main() {
	fmt.Println("a")
}
