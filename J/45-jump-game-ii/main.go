package main

import "fmt"

func jump(nums []int) int {
	k := 0
	buf := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		if nums[i]+i > k {
			k = nums[i] + i
			for j := i + 1; j <= k && j < len(nums); j++ {
				if buf[j] == 0 {
					buf[j] = buf[i] + 1
				} else if buf[j] > buf[i]+1 {
					buf[j] = buf[i] + 1
				}
			}
		}
	}
	return buf[len(buf)-1]
}

func jumpV2(nums []int) int {
	dp := make([]int, len(nums))
	dp[len(nums)-1] = 1

	for i := len(nums) - 2; i >= 0; i-- {
		min := len(nums) + 1
		isCan := false
		for j := 1; i+j < len(nums) && j <= nums[i]; j++ {
			if dp[i+j] >= 1 && dp[i+j]+1 < min {
				min = dp[i+j] + 1
				isCan = true
			}
		}
		if isCan {
			dp[i] = min
		}
	}

	return dp[0] - 1
}

func main() {
	fmt.Println("a")
}
