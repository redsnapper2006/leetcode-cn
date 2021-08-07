package main

import "fmt"

func circularArrayLoop(nums []int) bool {
	for i, v := range nums {
		idxSlow := (i + v) % len(nums)
		if idxSlow < 0 {
			idxSlow += len(nums)
		}
		idxFast := (idxSlow + nums[idxSlow]) % len(nums)
		if idxFast < 0 {
			idxFast += len(nums)
		}
		for idxFast != idxSlow {
			idxSlow = (idxSlow + nums[idxSlow]) % len(nums)
			if idxSlow < 0 {
				idxSlow += len(nums)
			}
			idxFast = (idxFast + nums[idxFast]) % len(nums)
			if idxFast < 0 {
				idxFast += len(nums)
			}
			idxFast = (idxFast + nums[idxFast]) % len(nums)
			if idxFast < 0 {
				idxFast += len(nums)
			}
		}
		startIdx := idxFast
		newIdx := (idxFast + nums[idxFast]) % len(nums)
		if newIdx < 0 {
			newIdx += len(nums)
		}
		forward := nums[idxFast] > 0
		isTarget := true
		steps := 1
		for newIdx != startIdx {
			if forward != (nums[newIdx] > 0) {
				isTarget = false
				break
			}
			newIdx = (newIdx + nums[newIdx]) % len(nums)
			if newIdx < 0 {
				newIdx += len(nums)
			}
			steps++
		}
		if steps == 1 {
			isTarget = false
		}
		if isTarget {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println()
}
