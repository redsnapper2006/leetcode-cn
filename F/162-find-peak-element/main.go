package main

import (
	"fmt"
)

func findPeakElement(nums []int) int {
	start := 0
	end := len(nums) - 1
	mid := (end + start) / 2

	if len(nums) == 1 {
		return 0
	}
	if len(nums) == 2 {
		if nums[1] > nums[0] {
			return 1
		} else {
			return 0
		}
	}
	for {
		if mid == 0 {
			if nums[mid] > nums[mid+1] {
				return mid
			} else {
				start = mid + 1
				mid = (end + start) / 2
				continue
			}
		}

		if mid == len(nums)-1 {
			if nums[mid] > nums[mid-1] {
				return mid
			} else {
				end = mid - 1
				mid = (end + start) / 2
				continue
			}
		}

		if nums[mid] > nums[mid-1] && nums[mid] > nums[mid+1] {
			return mid
		}

		if nums[mid] < nums[mid+1] {
			start = mid + 1
		} else {
			end = mid - 1
		}
		mid = (end + start) / 2
	}
}

func main() {
	fmt.Println(findPeakElement([]int{3, 4, 3, 2, 1}))
}
