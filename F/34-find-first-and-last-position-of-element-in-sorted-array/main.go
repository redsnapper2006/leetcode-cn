package main

import (
	"fmt"
)

func searchRange(nums []int, target int) []int {
	size := len(nums)

	if size == 0 {
		return []int{-1, -1}
	}
	if size == 1 {
		if nums[0] == target {
			return []int{0, 0}
		} else {
			return []int{-1, -1}
		}
	}

	start, end := 0, size-1
	for start <= end {
		if nums[(end+start)/2] >= target {
			end = (end+start)/2 - 1
		} else {
			start = (end+start)/2 + 1
		}
	}
	first := -1
	if end > 0 && nums[end] == target {
		first = end
	} else if start < size && nums[start] == target {
		first = start
	}

	start, end = 0, size-1
	for start <= end {
		if nums[(end+start)/2] <= target {
			start = (end+start)/2 + 1
		} else {
			end = (end+start)/2 - 1
		}
	}

	second := -1
	if end > 0 && nums[end] == target {
		second = end
	} else if start > 0 && nums[start-1] == target {
		second = start - 1
	}

	return []int{first, second}
}

func searchRangeV2(nums []int, target int) []int {
	size := len(nums)

	if size == 0 {
		return []int{-1, -1}
	}

	if size == 1 {
		if nums[0] == target {
			return []int{0, 0}
		} else {
			return []int{-1, -1}
		}
	}

	if size == 2 {
		if nums[0] == target && nums[1] == target {
			return []int{0, 1}
		}
		if nums[0] == target && nums[1] != target {
			return []int{0, 0}
		}
		if nums[0] != target && nums[1] == target {
			return []int{1, 1}
		}
		if nums[0] != target && nums[1] != target {
			return []int{-1, -1}
		}
	}

	half := size / 2
	if nums[half] > target {
		first := searchRange(nums[0:half], target)
		if first[0] == -1 {
			return first
		} else {
			return first
		}
	}
	if nums[half] < target {
		second := searchRange(nums[half+1:], target)
		if second[0] == -1 {
			return second
		} else {
			return []int{second[0] + half + 1, second[1] + half + 1}
		}
	}
	if nums[half] == target {
		first := searchRange(nums[0:half], target)
		second := searchRange(nums[half:], target)
		start, end := -1, -1
		if first[0] != -1 {
			start = first[0]
		} else {
			start = second[0] + half
		}
		if second[0] != -1 {
			end = second[1] + half
		} else {
			end = first[1]
		}
		return []int{start, end}
	}
	return nil
}

func main() {
	fmt.Println(searchRange([]int{5, 7, 7, 8, 8, 10}, 8))
	fmt.Println(searchRange([]int{5, 7, 7, 8, 8, 10}, 6))
}
