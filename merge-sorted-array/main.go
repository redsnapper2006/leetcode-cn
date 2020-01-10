package main

import (
	"fmt"
)

func merge(nums1 []int, m int, nums2 []int, n int) {
	n1count := m
	for _, n2 := range nums2 {

		var start, end int = 0, n1count
		for end > start {
			if nums1[(end+start)/2] > n2 {
				end = (end+start)/2 - 1
			} else if nums1[(end+start)/2] < n2 {
				start = (end+start)/2 + 1
			} else {
				start = (end + start) / 2
				break
			}
		}
		fmt.Println(start, end, n1count)
		if end < 0 {
			end = 0
		}
		if n1count > 0 && start > n1count-1 {
			start = n1count - 1
		}
		if n1count > 0 && nums1[start] < n2 {
			start++
		}

		fmt.Println(start, end, n1count)

		for i := n1count - 1; i >= start; i-- {
			nums1[i+1] = nums1[i]
		}
		nums1[start] = n2
		n1count++
		fmt.Println(nums1)
	}
}

func main() {
	fmt.Println("a")
	merge([]int{0, 0, 0, 0}, 0, []int{1, 2, 3}, 3)
	merge([]int{0, 0, 0, 0}, 0, []int{4, 2, 3}, 3)
	merge([]int{1, 2, 3, 0, 0, 0}, 3, []int{2, 5, 6}, 3)
}
