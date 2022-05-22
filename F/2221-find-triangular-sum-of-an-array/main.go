package main

func triangularSum(nums []int) int {
	for i := 0; i < len(nums); i++ {
		for j := 0; j < len(nums)-1-i; j++ {
			nums[j] = (nums[j] + nums[j+1])%10
		}
	}
	return nums[0] %10
}
