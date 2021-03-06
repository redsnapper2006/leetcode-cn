package main

import (
	"fmt"
)

func findKthLargest(nums []int, k int) int {
	pivot := nums[(len(nums) / 2)]
	start := 0
	end := len(nums) - 1
	for start <= end {
		for nums[start] > pivot {
			start++
		}
		for nums[end] < pivot {
			end--
		}
		if start < end {
			nums[start], nums[end] = nums[end], nums[start]
			start++
			end--
		} else if start == end {
			start++
		}
	}
	// fmt.Println(start, end, k, nums)
	if len(nums) == 2 && k == 1 {
		return nums[0]
	} else if end+1 > k {
		return findKthLargest(nums[0:end+1], k)
	} else if end+1 < k {
		return findKthLargest(nums[end+1:len(nums)], k-end-1)
	} else {
		return nums[end]
	}
}

func findKthLargestV2(nums []int, k int) int {
	buf := make([]int, k)
	copy(buf, nums[0:k])
	for i := k; i < len(nums); i++ {

		minIndex := 0
		for j := 1; j < len(buf); j++ {
			if buf[minIndex] > buf[j] {
				minIndex = j
			}
		}
		if nums[i] > buf[minIndex] {
			buf[minIndex] = nums[i]
		}
	}

	minIndex := 0
	for j := 1; j < len(buf); j++ {
		if buf[minIndex] > buf[j] {
			minIndex = j
		}
	}
	return buf[minIndex]
}

func main() {
	fmt.Println(findKthLargest([]int{3, 2, 1, 5, 6, 4}, 2))
	fmt.Println(findKthLargest([]int{3, 2, 3, 1, 2, 4, 5, 5, 6}, 4))
}
