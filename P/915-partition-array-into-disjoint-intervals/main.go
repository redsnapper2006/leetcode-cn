package main

func partitionDisjoint(nums []int) int {
	left, right := make([]int, len(nums)), make([]int, len(nums))

	leftMax, rightMin := nums[0], nums[len(nums)-1]
	for i := 0; i < len(nums); i++ {
		if nums[i] > leftMax {
			leftMax = nums[i]
		}
		if nums[len(nums)-1-i] < rightMin {
			rightMin = nums[len(nums)-1-i]
		}
		left[i] = leftMax
		right[len(nums)-1-i] = rightMin
	}
	for i := 0; i < len(nums)-1; i++ {
		if left[i] <= right[i+1] {
			return i + 1
		}
	}
	return -1
}
