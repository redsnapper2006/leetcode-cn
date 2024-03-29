package main

import "sort"

func minProductSum(nums1 []int, nums2 []int) int {
	sort.Ints(nums1)
	sort.Ints(nums2)

	sum := 0
	for i := 0; i < len(nums1); i++ {
		sum += nums1[i] * nums2[len(nums2)-1-i]
	}
	return sum
}
