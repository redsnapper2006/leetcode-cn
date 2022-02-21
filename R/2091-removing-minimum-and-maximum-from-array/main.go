package main

func minimumDeletions(nums []int) int {
	if len(nums) == 1 {
		return 1
	}

	min, max := 1000000, -1000000
	minIdx, maxIdx := -1, -1
	for i := 0; i < len(nums); i++ {
		if nums[i] < min {
			min = nums[i]
			minIdx = i
		}
		if nums[i] > max {
			max = nums[i]
			maxIdx = i
		}
	}

	if minIdx > maxIdx {
		minIdx, maxIdx = maxIdx, minIdx
	}
	ret := maxIdx + 1

	if ret > minIdx+1+len(nums)-maxIdx {
		ret = minIdx + 1 + len(nums) - maxIdx
	}
	if ret > len(nums)-minIdx {
		ret = len(nums) - minIdx
	}
	return ret
}
